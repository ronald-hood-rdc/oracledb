import express, { Express, Request, Response } from "express";
import { Client } from "@elastic/elasticsearch";

const app: Express = express();
const port = process.env.PORT || 3000;

app.get("/", (req: Request, res: Response) => {
  res.send("Hello, TypeScript Express!");
});

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});

// Make Call to Elastic Search
const client = new Client({
  cloud: {
    id: process.env.ELASTIC_CLOUD_ID!,
  },
  auth: {
    username: process.env.ELASTIC_USERNAME!,
    password: process.env.ELASTIC_PASSWORD!,
  },
});

const createIndex = async (indexName: string) => {
  await client.indices.create({ index: indexName });
  console.log("Index created");
};

createIndex("posts");
