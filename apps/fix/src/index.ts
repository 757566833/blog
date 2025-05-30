import express from 'express';
import protobuf from 'protobufjs';
import path from 'path';

const PORT = 4318;
const app = express();

// 接收 protobuf buffer
app.use(express.raw({ type: 'application/x-protobuf' }));

async function main() {
    // 加载 protobuf schema
    const root = new protobuf.Root();
    root.resolvePath = (origin, target) => {
        console.log(`origin: ${origin}, target: ${target}`);
        return path.join(__dirname,target) ;
    }
    root.loadSync("opentelemetry/proto/collector/logs/v1/logs_service.proto");

    const ExportLogsServiceRequest = root.lookupType(
        'opentelemetry.proto.collector.logs.v1.ExportLogsServiceRequest'
    );
    const ExportLogsServiceResponse = root.lookupType(
        'opentelemetry.proto.collector.logs.v1.ExportLogsServiceResponse'
    );

    app.post('/v1/logs', (req, res) => {
        try {
            const message = ExportLogsServiceRequest.decode(req.body);
            const object = ExportLogsServiceRequest.toObject(message, {
                longs: String,
                enums: String,
                bytes: String,
            });

            console.log('\n==== Received OTLP Logs ====');
            console.dir(object, { depth: null });

            // 返回一个空响应（表示成功）
            const responseBuffer = ExportLogsServiceResponse.encode({}).finish();
            res.setHeader('Content-Type', 'application/x-protobuf');
            res.status(200).send(responseBuffer);
        } catch (err) {
            console.error('Failed to decode Protobuf:', err);
            res.status(400).send('Invalid Protobuf');
        }
    });

    app.listen(PORT, () => {
        console.log(`OTLP HTTP log receiver listening on http://localhost:${PORT}/v1/logs`);
    });
}

main().catch(console.error);
