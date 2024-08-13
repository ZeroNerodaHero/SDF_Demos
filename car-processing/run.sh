fluvio cluster start
cd /workspace/connectors/ 
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg -c license-connector.yaml
cdk deploy start --ipkg infinyon-http-source-0.3.8.ipkg -c car-connector.yaml
cd /workspace
sdf run --ephemeral
fluvio consume cars
