#!/bin/bash

sea-orm-cli generate entity -u "sqlite://sample.db?mode=rwc" -o ./src/entity/
