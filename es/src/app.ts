import express, { Express, Request, Response } from "express";
import { Client } from "@elastic/elasticsearch";

const app: Express = express();
const port = process.env.PORT || 3000;

// Make Call to Elastic Search
const client = new Client({
  node: "https://vpc-customerdata-cache-dev-ixvlhp2x2vxk726vgu6qiv5vba.us-west-2.es.amazonaws.com",
});

app.get("/", async (req: Request, res: Response) => {
  try {
    console.log("hit", new Date().toLocaleTimeString());

    /*const { body } = await client.indices.getMapping({
      index: "party-contacts-index",
    });

    console.log("Mapping:", body);*/

    //const { body } = await client.cluster.health();

    const partyId = req.query.partyId as string;
    const elasticSearchQueryParams = {
      index: "party-contacts-index",
      body: {
        query: {
          match: {
            party_id: partyId,
          },
        },
        _source: ["*"],
      },
    };

    const { body } = await client.search(elasticSearchQueryParams);
    const hits = body.hits.hits;
    hits.forEach((hit: any) => {
      console.log(hit._source); // Log the retrieved fields
    });

    res.json(body);
  } catch (error) {
    console.error("Error connecting to Elasticsearch:", error);
    res.status(500).json({ error: "Error connecting to Elasticsearch" });
  }
});

app.listen(port, () => {
  console.log(`Server is running on port ${port}`);
});

/**
 * {"emails":{"properties":{"email_address":{"type":"text","fields":{"keyword":{"type":"keyword"}},"analyzer":"urls-links-emails","fielddata":true},"status":{"type":"keyword"}}},"party_id":{"type":"keyword"},"party_name":{"type":"text","analyzer":"name_synonym_analyzer","fielddata":true},"phones":{"properties":{"phone_number":{"type":"text","fields":{"keyword":{"type":"keyword","ignore_above":256}}
 */
