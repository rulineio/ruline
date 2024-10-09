schema "ruline" {
  charset = "utf8mb4"
  collate = "utf8mb4_general_ci"
}

table "users" {
  schema = schema.ruline
  column "id" {
      type = char(30)
  }
  column "email" {
      type = varchar(255)
  }
  column "name" {
      type = varchar(255)
  }
  column "status" {
      type = enum("created", "active")
      default = "created"
  }
  column "avatar" {
      type = varchar(255)
  }
  column "created_at" {
      type = timestamp
      default = sql("CURRENT_TIMESTAMP")
  }
  column "updated_at" {
      type = timestamp
      default = sql("CURRENT_TIMESTAMP")
      on_update = sql("CURRENT_TIMESTAMP")
  }
  column "last_login" {
      type = timestamp
      default = sql("CURRENT_TIMESTAMP")
  }

  primary_key {
      columns = [column.id]
  }

  index "idx_users_email" {
      columns = [column.email]
      unique = true
  }
}

table "organizations" {
  schema = schema.ruline
  column "id" {
    type = char(30)
  }
  column "name" {
    type = varchar(255)
  }
  column "status" {
    type = enum("active")
    default = "active"
  }
  column "avatar" {
    type = varchar(255)
  }
  column "created_at" {
    type = timestamp
    default = sql("CURRENT_TIMESTAMP")
  }
  column "updated_at" {
    type = timestamp
    default = sql("CURRENT_TIMESTAMP")
    on_update = sql("CURRENT_TIMESTAMP")
  }

  primary_key {
    columns = [column.id]
  }
}

table "members" {
  schema = schema.ruline
  column "user_id" {
    type = char(30)
  }
  column "organization_id" {
    type = char(30)
  }
  column "role" {
    type = enum("owner", "admin", "editor", "viewer", "member")
    default = "member"
  }
  column "status" {
    type = enum("active", "left", "removed")
    default = "active"
  }
  column "created_at" {
    type = timestamp
    default = sql("CURRENT_TIMESTAMP")
  }
  column "updated_at" {
    type = timestamp
    default = sql("CURRENT_TIMESTAMP")
    on_update = sql("CURRENT_TIMESTAMP")
  }

  primary_key {
    columns = [column.user_id, column.organization_id]
  }

  foreign_key "user_id" {
    columns = [column.user_id]
    ref_columns = [table.users.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  foreign_key "organization_id" {
    columns = [column.organization_id]
    ref_columns = [table.organizations.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
}
