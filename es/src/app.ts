import express, { Express, Request, Response } from "express";
import { Client } from "@elastic/elasticsearch";

const app: Express = express();
const port = process.env.PORT || 3000;

// Make Call to Elastic Search
const client = new Client({
  node: "https://vpc-customerdata-cache-dev-ixvlhp2x2vxk726vgu6qiv5vba.us-west-2.es.amazonaws.com",
});

app.get("/", async (req: Request, res: Response) => {
  const partyId = req.query.party_id;
  try {
    console.log("hit", new Date().toLocaleTimeString());

    const elasticSearchQueryParams = {
      index: "party-contacts-index",
      body: {
        query: {
          match: {
            party_id: partyId,
          },
        },
        _source: ["party_id", "party_info"],
      },
    };

    const { body } = await client.search(elasticSearchQueryParams);
    res.json(body);
  } catch (error) {
    console.error("Error connecting to Elasticsearch:", error);
    res.status(500).json({ error: "Error connecting to Elasticsearch" });
  }
});

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});
