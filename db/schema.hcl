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
  column "logo" {
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
  column "id" {
    type = char(30)
  }
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
    type = enum("active", "left", "removed", "invited", "declined")
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
    columns = [column.id]
  }

  foreign_key "member_user_id" {
    columns = [column.user_id]
    ref_columns = [table.users.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  foreign_key "member_organization_id" {
    columns = [column.organization_id]
    ref_columns = [table.organizations.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  index "idx_members_user_id_status" {
    columns = [column.user_id, column.status]
    unique = true
  }

  index "idx_members_organization_id" {
    columns = [column.organization_id]
  }
}

table "invitations" {
  schema = schema.ruline
  column "id" {
    type = char(30)
  }
  column "organization_id" {
    type = char(30)
  }
  column "user_id" {
    type = varchar(30)
  }
  column "member_id" {
    type = char(30)
  }
  column "status" {
    type = enum("created", "accepted", "declined")
    default = "created"
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

  foreign_key "invitation_organization_id" {
    columns = [column.organization_id]
    ref_columns = [table.organizations.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  foreign_key "invitation_user_id" {
    columns = [column.user_id]
    ref_columns = [table.users.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  foreign_key "invitation_member_id" {
    columns = [column.member_id]
    ref_columns = [table.members.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  index "idx_invitations_user_id_status" {
    columns = [column.user_id, column.status]
  }
}

table "projects" {
  schema = schema.ruline
  column "id" {
    type = char(30)
  }
  column "organization_id" {
    type = char(30)
  }
  column "name" {
    type = varchar(255)
  }
  column "status" {
    type = enum("active")
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
    columns = [column.organization_id, column.id]
  }

  foreign_key "project_organization_id" {
    columns = [column.organization_id]
    ref_columns = [table.organizations.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }
}

table "workflows" {
  schema = schema.ruline
  column "id" {
    type = char(30)
  }
  column "organization_id" {
    type = char(30)
  }
  column "project_id" {
    type = char(30)
  }
  column "name" {
    type = varchar(255)
  }
  column "status" {
    type = enum("active", "archived")
    default = "active"
  }
  column "active_version" {
    type = mediumint
    unsigned = true
    null = true
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
    columns = [column.organization_id, column.project_id, column.id]
  }

  foreign_key "workflow_project_id" {
    columns = [column.organization_id, column.project_id]
    ref_columns = [table.projects.column.organization_id, table.projects.column.id]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  foreign_key "workflow_active_version" {
    columns = [column.organization_id, column.project_id, column.id, column.active_version]
    ref_columns = [
      table.workflow_versions.column.organization_id,
      table.workflow_versions.column.project_id,
      table.workflow_versions.column.workflow_id,
      table.workflow_versions.column.version
    ]
    on_update = NO_ACTION
    on_delete = NO_ACTION
  }

  index "idx_workflows_organization_id" {
    columns = [column.organization_id, column.id, column.status]
  }
}

table "workflow_versions" {
  schema = schema.ruline
  column "organization_id" {
    type = char(30)
  }
  column "project_id" {
    type = char(30)
  }
  column "workflow_id" {
    type = char(30)
  }
  column "version" {
    type = mediumint
    unsigned = true
  }
  column "status" {
    type = enum("draft", "in_review", "published", "archived")
    default = "draft"
  }
  column "definition" {
    type = json
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
    columns = [column.organization_id, column.project_id, column.workflow_id, column.version]
  }

  foreign_key "workflow_version_workflow_id" {
    columns = [column.organization_id, column.project_id, column.workflow_id]
    ref_columns = [
      table.workflows.column.organization_id,
      table.workflows.column.project_id,
      table.workflows.column.id
    ]
    on_update = NO_ACTION
    on_delete = CASCADE
  }

  index "idx_workflow_versions_workflow_id_version" {
    columns = [column.workflow_id, column.version, column.status]
  }
}
