_list:
  @just --list

# Stage all the current files in git, then run Filigree
build-with-backup:
  git add .
  @just build-test-apps

build-test-apps *FLAGS:
  cd ../filigree/filigree-cli && cargo lbuild
  just build-test-app sveltekit
  just build-test-app htmx
  just build-test-app custom_auth_string_ids

build-test-app DIR *FLAGS:
  @just write-files {{DIR}} {{FLAGS}}
  cd {{DIR}} && cargo lcheck

write-files DIR *FLAGS:
  cd ../filigree/filigree-cli && cargo lbuild
  cd {{DIR}} && ../../filigree/target/debug/filigree write {{FLAGS}}

build-and-test DIR *FLAGS:
  @just build-test-app {{DIR}}
  cd test-apps/{{DIR}} && cargo ltest {{FLAGS}}

build-test-app-and-db DIR *FLAGS:
  cd filigree-cli && cargo lbuild
  cd {{DIR}} && ../../filigree/target/debug/filigree write --overwrite && (yes | sqlx database reset) && cargo ltest {{FLAGS}}

build-web-types:
  true
