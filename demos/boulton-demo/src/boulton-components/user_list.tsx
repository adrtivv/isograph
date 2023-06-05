import * as React from "react";
import { bDeclare, read } from "@boulton/react";

export const user_list_page = bDeclare`
  Query.user_list_page(
    $bar: String!,
    $bar2: String!,
  ) @fetchable {
    byah(foo: $bar,),
    users {
      id,
      user_list_component,
    },
    some_resolver,
  }
`;

export const some_resolver = bDeclare`
  Query.some_resolver @eager {
    byah(foo: $bar2,),
  }
`;

export const user_list_component = bDeclare`
  User.user_list_component @component {
    id,
    name,
    avatar_component,
  }
`(UserListComponent);

function UserListComponent({ data, onSelectId }) {
  return (
    <>
      <h2>{data.name}</h2>
      {data.avatar_component()}
      <button onClick={() => onSelectId(data.id)}>User details</button>
    </>
  );
}
