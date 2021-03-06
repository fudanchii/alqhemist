# DO NOT EDIT! THIS FILE IS AUTO-GENERATED FROM A GRAPHQL SCHEMA
module Types
  class DType < Types::BaseObject
    field :servant_class, Types::ServantClassEnum, null: false,
      description: "Servant class name"
    field :weapons, [String], null: false,
      description: "List of weapon used by this servant"
    field :alters, [String, null: true], null: false,
      description: "List of alter characters"
  end
end
