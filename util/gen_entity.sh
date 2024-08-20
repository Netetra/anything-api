#!/bin/bash

sea-orm-cli generate entity --with-serde both -u "sqlite://sample.db?mode=rwc" -o ./src/entity/
