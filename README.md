# docker-compose-profile-menu
This utility ingests docker-compose yaml files and lets a user select which profiles they want.

The only command line parameter is the path to look in for docker-compose.yaml and docker-compose.override.yaml.  When found, all the `profiles` for each service are discovered and then presented to the user so they can select which profiles they want active.  This could then be used with the [COMPOSE_PROFILES](https://docs.docker.com/compose/profiles/) env or used in the ``--profiles`` argument to `docker compose`.

The final line of output is a comma separated list of the selected profiles.

## Example
```
> docker-compose-profile-menu .
? Select profiles (space to select, enter to confirm) ›
✔ debug
✔ frontend
  backend
debug,frontend
```
