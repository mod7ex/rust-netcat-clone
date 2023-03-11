rm *.csr;
rm *.pem;

cfssl gencert -initca ca-csr.json | cfssljson -bare ca
cfssl gencert -ca ca.pem -ca-key ca-key.pem -config ca-config.json -profile server server.json | cfssljson -bare server