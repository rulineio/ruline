schema "ruline" {
  charset = "utf8mb4"
  collate = "utf8mb4_general_ci"
}

table "users" {
  schema = schema.ruline
  column "id" {
      type = char(26)
  }
  column "email" {
      type = varchar(255)
  }
  column "name" {
      type = varchar(255)
  }
  column "status" {
      type = enum("created", "active", "blocked")
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
