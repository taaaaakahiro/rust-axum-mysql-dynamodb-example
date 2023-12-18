const { DynamoDBClient, BatchWriteItemCommand } = require("@aws-sdk/client-dynamodb");
const { v4: uuidv4 } = require('uuid');

(async () => {
    const client = new DynamoDBClient({
        region: "ap-northeast-1",
        endpoint: "http://localhost:8000"
    });
    const items = [
        {
            "id": { S: uuidv4() },
            "Name": { S: "name1" }
        },
        {
            "id": { S: uuidv4() },
            "Name": { S: "name2" }
        },
        // Add more items as needed
    ];

    const putRequests = items.map(item => ({
        PutRequest: {
            Item: item
        }
    }));

    const command = new BatchWriteItemCommand({
        RequestItems: {
            'examples': putRequests
        }
    });
    try {
        const results = await client.send(command);
        console.log('result:', results);
    } catch (err) {
        console.error(err);
    }
})();
