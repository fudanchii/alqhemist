# DO NOT EDIT! THIS FILE IS AUTO-GENERATED FROM A GRAPHQL SCHEMA
module Types
  class CType < Types::BaseObject
    field :name, String, null: false
    field :address, String, null: false do
      argument :abbreviate, Boolean, required: true, default_value: false
    end
  end
end
