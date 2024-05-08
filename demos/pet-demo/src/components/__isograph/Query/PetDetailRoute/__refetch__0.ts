import type {IsographEntrypoint, ReaderAst, FragmentReference, NormalizationAst, RefetchQueryNormalizationArtifact} from '@isograph/react';
const queryText = 'query Pet_refetch ($id: ID!) { node____id___id: node(id: $id) { ... on Pet { \
  id,\
  age,\
  best_friend_relationship {\
    best_friend {\
      id,\
      name,\
      picture,\
    },\
    picture_together,\
  },\
  checkins {\
    id,\
    location,\
    time,\
  },\
  favorite_phrase,\
  name,\
  nickname,\
  potential_new_best_friends {\
    id,\
    name,\
  },\
  stats {\
    cuteness,\
    energy,\
    hunger,\
    intelligence,\
    sociability,\
    weight,\
  },\
  tagline,\
}}}';

const normalizationAst: NormalizationAst = [{ kind: "Linked", fieldName: "node", arguments: [[ "id", { kind: "Variable", name: "id" }]], selections: [
  {
    kind: "Scalar",
    fieldName: "id",
    arguments: null,
  },
  {
    kind: "Scalar",
    fieldName: "age",
    arguments: null,
  },
  {
    kind: "Linked",
    fieldName: "best_friend_relationship",
    arguments: null,
    selections: [
      {
        kind: "Linked",
        fieldName: "best_friend",
        arguments: null,
        selections: [
          {
            kind: "Scalar",
            fieldName: "id",
            arguments: null,
          },
          {
            kind: "Scalar",
            fieldName: "name",
            arguments: null,
          },
          {
            kind: "Scalar",
            fieldName: "picture",
            arguments: null,
          },
        ],
      },
      {
        kind: "Scalar",
        fieldName: "picture_together",
        arguments: null,
      },
    ],
  },
  {
    kind: "Linked",
    fieldName: "checkins",
    arguments: null,
    selections: [
      {
        kind: "Scalar",
        fieldName: "id",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "location",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "time",
        arguments: null,
      },
    ],
  },
  {
    kind: "Scalar",
    fieldName: "favorite_phrase",
    arguments: null,
  },
  {
    kind: "Scalar",
    fieldName: "name",
    arguments: null,
  },
  {
    kind: "Scalar",
    fieldName: "nickname",
    arguments: null,
  },
  {
    kind: "Linked",
    fieldName: "potential_new_best_friends",
    arguments: null,
    selections: [
      {
        kind: "Scalar",
        fieldName: "id",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "name",
        arguments: null,
      },
    ],
  },
  {
    kind: "Linked",
    fieldName: "stats",
    arguments: null,
    selections: [
      {
        kind: "Scalar",
        fieldName: "cuteness",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "energy",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "hunger",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "intelligence",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "sociability",
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "weight",
        arguments: null,
      },
    ],
  },
  {
    kind: "Scalar",
    fieldName: "tagline",
    arguments: null,
  },
] }];
const artifact: any = {
  kind: "RefetchQuery",
  queryText,
  normalizationAst,
};

export default artifact;
