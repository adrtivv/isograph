use std::collections::HashMap;

use common_lang_types::{
    DefinedField, DescriptionValue, HasName, InputTypeId, InputTypeName, JavascriptName, ObjectId,
    ObjectTypeName, OutputTypeId, OutputTypeName, ResolverDefinitionPath, ResolverFieldId,
    ScalarFieldName, ScalarId, ScalarTypeName, ServerFieldDefinitionName, ServerFieldId,
    TypeAndField, TypeId, TypeWithFieldsId, TypeWithFieldsName, TypeWithoutFieldsId,
    TypeWithoutFieldsName, UnvalidatedTypeName, ValidLinkedFieldType, ValidScalarFieldType,
    ValidTypeAnnotationInnerType, WithSpan,
};
use graphql_lang_types::TypeAnnotation;
use intern::string_key::Intern;
use isograph_lang_types::{Selection, Unwrap, VariableDefinition};
use lazy_static::lazy_static;

use crate::ResolverVariant;

lazy_static! {
    pub static ref STRING_JAVASCRIPT_TYPE: JavascriptName = "string".intern().into();
}

/// The first, unvalidated in-memory representation of a schema.
///
/// The things that are unvalidated include:
/// - That each field's type exists
/// - That each resolver's fragment is valid, i.e. that fields
///   exist, no duplicates, etc.
///
/// This is almost certainly a subset of validations we should do.
///
/// Invariant: a schema is append-only, because pointers into the Schema are in the
/// form of newtype wrappers around u32 indexes (e.g. FieldId, etc.) As a result,
/// the schema does not support removing items.
///
/// TServerType: the type of a parsed or validated server field in the fields array.
/// In an unvalidated schema, this is UnvalidatedTypeName. In a validated schema,
/// this is OutputTypeId.
#[derive(Debug)]
pub struct Schema<
    TServerType: ValidTypeAnnotationInnerType,
    TScalarField: ValidScalarFieldType,
    TLinkedField: ValidLinkedFieldType,
    TVariableType: ValidTypeAnnotationInnerType,
    // On objects, what does the HashMap of encountered types contain
    TEncounteredField,
> {
    pub fields: Vec<SchemaServerField<TypeAnnotation<TServerType>>>,
    pub resolvers: Vec<SchemaResolver<TScalarField, TLinkedField, TVariableType>>,
    pub schema_data: SchemaData<TEncounteredField>,

    // Well known types
    pub id_type_id: ScalarId,
    pub string_type_id: ScalarId,
    pub float_type_id: ScalarId,
    pub boolean_type_id: ScalarId,
    pub int_type_id: ScalarId,

    // typename
    pub query_type_id: Option<ObjectId>,
    // Subscription
    // Mutation
}

pub(crate) type UnvalidatedSchema =
    Schema<UnvalidatedTypeName, (), (), UnvalidatedTypeName, UnvalidatedObjectFieldInfo>;

/// On unvalidated schema objects, the encountered types are either a type annotation
/// for server fields with an unvalidated inner type, or a ScalarFieldName (the name of the
/// resolver.)
pub type UnvalidatedObjectFieldInfo =
    DefinedField<TypeAnnotation<UnvalidatedTypeName>, ScalarFieldName>;

pub(crate) type UnvalidatedSchemaData = SchemaData<UnvalidatedObjectFieldInfo>;

pub(crate) type UnvalidatedSchemaField = SchemaServerField<TypeAnnotation<UnvalidatedTypeName>>;

pub(crate) type UnvalidatedSchemaResolver = SchemaResolver<(), (), UnvalidatedTypeName>;

#[derive(Debug)]
pub struct SchemaData<TEncounteredField> {
    pub objects: Vec<SchemaObject<TEncounteredField>>,
    pub scalars: Vec<SchemaScalar>,
    // enums, unions, interfaces, input objects
    pub defined_types: HashMap<UnvalidatedTypeName, TypeId>,
}

impl<
        TServerType: ValidTypeAnnotationInnerType,
        TScalarField: ValidScalarFieldType,
        TLinkedField: ValidLinkedFieldType,
        TVariableType: ValidTypeAnnotationInnerType,
        TEncounteredField,
    > Schema<TServerType, TScalarField, TLinkedField, TVariableType, TEncounteredField>
{
    pub fn field(
        &self,
        field_id: ServerFieldId,
    ) -> &SchemaServerField<TypeAnnotation<TServerType>> {
        &self.fields[field_id.as_usize()]
    }

    pub fn resolver(
        &self,
        resolver_field_id: ResolverFieldId,
    ) -> &SchemaResolver<TScalarField, TLinkedField, TVariableType> {
        &self.resolvers[resolver_field_id.as_usize()]
    }

    pub fn query_object(&self) -> Option<&SchemaObject<TEncounteredField>> {
        self.query_type_id
            .as_ref()
            .map(|id| self.schema_data.object(*id))
    }
}

impl UnvalidatedSchema {
    pub fn new() -> Self {
        // TODO add __typename
        let fields = vec![];
        let resolvers = vec![];
        let objects = vec![];
        let mut scalars = vec![];
        let mut defined_types = HashMap::default();

        let id_type_id = add_schema_defined_scalar_type(
            &mut scalars,
            &mut defined_types,
            "ID",
            *STRING_JAVASCRIPT_TYPE,
        );
        let string_type_id = add_schema_defined_scalar_type(
            &mut scalars,
            &mut defined_types,
            "String",
            *STRING_JAVASCRIPT_TYPE,
        );
        let boolean_type_id = add_schema_defined_scalar_type(
            &mut scalars,
            &mut defined_types,
            "Boolean",
            "boolean".intern().into(),
        );
        let float_type_id = add_schema_defined_scalar_type(
            &mut scalars,
            &mut defined_types,
            "Float",
            "number".intern().into(),
        );
        let int_type_id = add_schema_defined_scalar_type(
            &mut scalars,
            &mut defined_types,
            "Int",
            "number".intern().into(),
        );

        Self {
            fields,
            resolvers,
            schema_data: SchemaData {
                objects,
                scalars,
                defined_types,
            },

            id_type_id,
            string_type_id,
            int_type_id,
            float_type_id,
            boolean_type_id,

            query_type_id: None,
        }
    }
}

impl<TEncounteredField> SchemaData<TEncounteredField> {
    pub fn lookup_type_with_fields(
        &self,
        type_id: TypeWithFieldsId,
    ) -> SchemaTypeWithFields<TEncounteredField> {
        match type_id {
            TypeWithFieldsId::Object(object_id) => {
                // TODO replace with an unchecked lookup?
                SchemaTypeWithFields::Object(&self.objects[object_id.as_usize()])
            }
        }
    }

    pub fn lookup_type_without_fields(
        &self,
        type_without_fields_id: TypeWithoutFieldsId,
    ) -> SchemaTypeWithoutFields {
        match type_without_fields_id {
            TypeWithoutFieldsId::Scalar(scalar_id) => {
                SchemaTypeWithoutFields::Scalar(self.scalar(scalar_id))
            }
        }
    }

    pub fn scalar(&self, scalar_id: ScalarId) -> &SchemaScalar {
        self.scalars
            .get(scalar_id.as_usize())
            .expect("Invalid ScalarId")
    }

    pub fn lookup_unvalidated_type(&self, type_id: TypeId) -> SchemaType<TEncounteredField> {
        match type_id {
            TypeId::Object(id) => SchemaType::Object(self.objects.get(id.as_usize()).unwrap()),
            TypeId::Scalar(id) => SchemaType::Scalar(self.scalars.get(id.as_usize()).unwrap()),
        }
    }

    pub fn lookup_output_type(
        &self,
        output_type_id: OutputTypeId,
    ) -> SchemaOutputType<TEncounteredField> {
        match output_type_id {
            OutputTypeId::Object(id) => {
                SchemaOutputType::Object(self.objects.get(id.as_usize()).unwrap())
            }
            OutputTypeId::Scalar(id) => {
                SchemaOutputType::Scalar(self.scalars.get(id.as_usize()).unwrap())
            }
        }
    }

    pub fn lookup_input_type(&self, input_type_id: InputTypeId) -> SchemaInputType {
        match input_type_id {
            InputTypeId::Scalar(id) => {
                SchemaInputType::Scalar(self.scalars.get(id.as_usize()).unwrap())
            }
        }
    }

    pub fn object(&self, object_id: ObjectId) -> &SchemaObject<TEncounteredField> {
        self.objects
            .get(object_id.as_usize())
            .expect("ObjectId should exist, this indicates a bug in Isograph")
    }
}

fn add_schema_defined_scalar_type(
    scalars: &mut Vec<SchemaScalar>,
    defined_types: &mut HashMap<UnvalidatedTypeName, TypeId>,
    field_name: &'static str,
    javascript_name: JavascriptName,
) -> ScalarId {
    let scalar_id = scalars.len().into();

    let typename = field_name.intern().into();
    scalars.push(SchemaScalar {
        description: None,
        name: typename,
        id: scalar_id,
        javascript_name,
    });
    defined_types.insert(typename.into(), TypeId::Scalar(scalar_id.into()));
    scalar_id
}

#[derive(Debug)]
pub enum SchemaTypeWithFields<'a, TEncounteredField> {
    Object(&'a SchemaObject<TEncounteredField>),
}

impl<'a, TEncounteredField> Copy for SchemaTypeWithFields<'a, TEncounteredField> {}
impl<'a, TEncounteredField> Clone for SchemaTypeWithFields<'a, TEncounteredField> {
    fn clone(&self) -> Self {
        match self {
            Self::Object(arg0) => Self::Object(arg0),
        }
    }
}

pub type UnvalidatedSchemaTypeWithFields<'a> = SchemaTypeWithFields<'a, UnvalidatedObjectFieldInfo>;

impl<'a, TEncounteredField> From<&'a SchemaObject<TEncounteredField>>
    for SchemaTypeWithFields<'a, TEncounteredField>
{
    fn from(object: &'a SchemaObject<TEncounteredField>) -> Self {
        SchemaTypeWithFields::Object(object)
    }
}

impl<'a, TEncounteredField> SchemaTypeWithFields<'a, TEncounteredField> {
    pub fn encountered_field_names(
        &self,
    ) -> &HashMap<ServerFieldDefinitionName, TEncounteredField> {
        match self {
            SchemaTypeWithFields::Object(object) => &object.encountered_field_names,
        }
    }

    pub fn fields(&self) -> &[ServerFieldId] {
        match self {
            SchemaTypeWithFields::Object(object) => &object.fields,
        }
    }

    pub fn resolvers(&self) -> &[ResolverFieldId] {
        match self {
            SchemaTypeWithFields::Object(object) => &object.resolvers,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SchemaType<'a, TEncounteredField> {
    Object(&'a SchemaObject<TEncounteredField>),
    Scalar(&'a SchemaScalar),
    // Includes input object
}

impl<'a, TEncounteredField> HasName for SchemaTypeWithFields<'a, TEncounteredField> {
    type Name = TypeWithFieldsName;

    fn name(&self) -> Self::Name {
        match self {
            SchemaTypeWithFields::Object(object) => object.name.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SchemaOutputType<'a, TEncounteredField> {
    Object(&'a SchemaObject<TEncounteredField>),
    Scalar(&'a SchemaScalar),
    // excludes input object
}

impl<'a, TEncounteredField> HasName for SchemaOutputType<'a, TEncounteredField> {
    type Name = OutputTypeName;

    fn name(&self) -> Self::Name {
        match self {
            SchemaOutputType::Object(object) => object.name.into(),
            SchemaOutputType::Scalar(scalar) => scalar.name.into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SchemaInputType<'a> {
    Scalar(&'a SchemaScalar),
    // input object
    // enum
}

impl<'a> HasName for SchemaInputType<'a> {
    type Name = InputTypeName;

    fn name(&self) -> Self::Name {
        match self {
            SchemaInputType::Scalar(x) => (x.name).into(),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum SchemaTypeWithoutFields<'a> {
    Scalar(&'a SchemaScalar),
    // enum
}

impl<'a> HasName for SchemaTypeWithoutFields<'a> {
    type Name = TypeWithoutFieldsName;

    fn name(&self) -> Self::Name {
        match self {
            SchemaTypeWithoutFields::Scalar(scalar) => scalar.name.into(),
        }
    }
}

impl<'schema> SchemaTypeWithoutFields<'schema> {
    pub fn javascript_name(&self) -> JavascriptName {
        match self {
            SchemaTypeWithoutFields::Scalar(scalar) => scalar.javascript_name,
        }
    }
}

#[derive(Debug)]
pub struct SchemaObject<TEncounteredField> {
    pub description: Option<DescriptionValue>,
    pub name: ObjectTypeName,
    pub id: ObjectId,
    // pub interfaces: Vec<InterfaceTypeName>,
    // pub directives: Vec<Directive<ConstantValue>>,
    pub fields: Vec<ServerFieldId>,
    pub resolvers: Vec<ResolverFieldId>,
    pub encountered_field_names: HashMap<ServerFieldDefinitionName, TEncounteredField>,
}
// Unvalidated => TScalarField: TypeAnnotation<UnvalidatedTypeName>,
// Validated => FieldId

#[derive(Debug)]
pub struct SchemaServerField<T> {
    pub description: Option<DescriptionValue>,
    pub name: ServerFieldDefinitionName,
    pub id: ServerFieldId,
    pub field_type: T,
    pub parent_type_id: TypeWithFieldsId,
    // pub arguments: Vec<InputValue<ConstantValue>>,
    // pub directives: Vec<Directive<ConstantValue>>,
}

#[derive(Debug)]
pub struct SchemaResolver<
    TScalarField: ValidScalarFieldType,
    TLinkedField: ValidLinkedFieldType,
    TVariableDefinitionType: ValidTypeAnnotationInnerType,
> {
    pub description: Option<DescriptionValue>,
    pub name: ServerFieldDefinitionName,
    pub id: ResolverFieldId,
    pub resolver_definition_path: ResolverDefinitionPath,
    // TODO it makes no sense for a resolver to not select fields!
    // Why not just make it a global function at that point? Who knows.
    // Unless you'll eventually select fields?
    pub selection_set_and_unwraps: Option<(
        Vec<WithSpan<Selection<TScalarField, TLinkedField>>>,
        Vec<WithSpan<Unwrap>>,
    )>,
    pub variant: Option<WithSpan<ResolverVariant>>,
    // This should not be a bool; this should be an enum?
    pub is_fetchable: bool,
    pub variable_definitions: Vec<WithSpan<VariableDefinition<TVariableDefinitionType>>>,

    // Why is this not calculated when needed?
    pub type_and_field: TypeAndField,
    pub has_associated_js_function: bool,

    // TODO should this be TypeWithFieldsId???
    pub parent_object_id: ObjectId,
}

impl<T> SchemaServerField<T> {
    // TODO probably unnecessary, and can be replaced with .map and .transpose
    pub fn split(self) -> (SchemaServerField<()>, T) {
        let Self {
            description,
            name,
            id,
            field_type,
            parent_type_id,
        } = self;
        (
            SchemaServerField {
                description,
                name,
                id,
                field_type: (),
                parent_type_id,
            },
            field_type,
        )
    }
}

#[derive(Debug)]
pub struct SchemaScalar {
    pub description: Option<DescriptionValue>,
    pub name: ScalarTypeName,
    pub id: ScalarId,
    pub javascript_name: JavascriptName,
    // pub directives: Vec<Directive<ConstantValue>>,
}
