import type {ComponentReaderArtifact, ExtractSecondParam, ReaderAst, RefetchQueryNormalizationArtifact} from '@isograph/react';
import { Pet__PetUpdater__param } from './param_type';
import { PetUpdater as resolver } from '../../../PetUpdater.tsx';
import Pet____refetch from '../__refetch/reader';
import Pet__set_best_friend from '../set_best_friend/reader';
import Pet__set_pet_tagline from '../set_pet_tagline/reader';

const readerAst: ReaderAst<Pet__PetUpdater__param> = [
  {
    kind: "MutationField",
    alias: "set_best_friend",
    // @ts-ignore
    readerArtifact: Pet__set_best_friend,
    refetchQuery: 1,
  },
  {
    kind: "Linked",
    fieldName: "potential_new_best_friends",
    alias: null,
    arguments: null,
    selections: [
      {
        kind: "Scalar",
        fieldName: "id",
        alias: null,
        arguments: null,
      },
      {
        kind: "Scalar",
        fieldName: "name",
        alias: null,
        arguments: null,
      },
    ],
  },
  {
    kind: "MutationField",
    alias: "set_pet_tagline",
    // @ts-ignore
    readerArtifact: Pet__set_pet_tagline,
    refetchQuery: 2,
  },
  {
    kind: "Scalar",
    fieldName: "tagline",
    alias: null,
    arguments: null,
  },
  {
    kind: "RefetchField",
    alias: "__refetch",
    readerArtifact: Pet____refetch,
    refetchQuery: 0,
  },
];

const artifact: ComponentReaderArtifact<
  Pet__PetUpdater__param,
  ExtractSecondParam<typeof resolver>
> = {
  kind: "ComponentReaderArtifact",
  componentName: "Pet.PetUpdater",
  resolver,
  readerAst,
};

export default artifact;
