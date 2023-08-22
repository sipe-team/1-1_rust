-- Add migration script here
CREATE TABLE "Board" (
  "id" serial PRIMARY KEY,
  "name" text UNIQUE NOT NULL
);

CREATE TABLE "Swimlane" (
  "id" serial PRIMARY KEY,
  "board_id" int NOT NULL,
  "name" text UNIQUE NOT NULL,
  "description" text
);

CREATE TABLE "Ticket" (
  "id" serial PRIMARY KEY,
  "swimlane_id" int NOT NULL,
  "name" text UNIQUE NOT NULL,
  "description" text,
  "start_date" timestamp,
  "end_date" timestamp,
  "priority" int NOT NULL
);

CREATE INDEX idx_board_name ON "Board" ("name");

CREATE INDEX idx_swimlane_board_id ON "Swimlane" ("board_id");
CREATE INDEX idx_swimlane_name ON "Swimlane" ("name");

CREATE INDEX idx_ticket_swimlane_id ON "Ticket" ("swimlane_id");
CREATE INDEX idx_ticket_name ON "Ticket" ("name");
CREATE INDEX idx_ticket_priority ON "Ticket" ("priority");