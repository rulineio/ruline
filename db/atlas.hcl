env "ruline" {
    src = "file://db/schema.hcl"
    url = getenv("DATABASE_URL")
    dev = "docker://maria/latest"

    migration {
        dir = "file://db/migrations"
    }
}
