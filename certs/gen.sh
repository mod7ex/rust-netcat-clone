rm *.pem;
rm *.srl;

# 1. Generate CA's private key and self-signed certificate [Certification authority]
###### Only for Dev (no passphrase)
# openssl req -x509 -newkey rsa:4096 -days 356 -nodes -keyout ca-key.pem -out ca-cert.pem -subj "/C=MA/ST=Casablanca-Settate/L=Casablanca/O=Adnovado/OU=Tech/CN=*.adnovado.com/emailAddress=mail@adnovado.com";
openssl req -x509 -newkey rsa:4096 -days 356 -keyout ca-key.pem -out ca-cert.pem -subj "/C=US/ST=California/L=Colorado/O=CaProvider/OU=Tech/CN=*.ca-pros.com/emailAddress=mail@ca-pros.com";

# display certificate info
echo "CA's self-signed certificate";
openssl x509 -in ca-cert.pem -noout -text;

# 2. Generate web server's private key and certificate signing request (CSR) [Individual]
###### Only for Dev (no passphrase)
# openssl req -newkey rsa:4096 -nodes -keyout server-key.pem -out server-req.pem -subj "/C=MA/ST=Casablanca-Settate/L=Casablanca/O=Adnovado/OU=Tech/CN=*.adnovado.com/emailAddress=mail@adnovado.com";
openssl req -newkey rsa:4096 -keyout server-key.pem -out server-req.pem -subj "/C=MA/ST=Casablanca-Settate/L=Casablanca/O=Adnovado/OU=Tech/CN=*.adnovado.com/emailAddress=mail@adnovado.com";

# 3. Use CA's private key to sign web server's CSR and get back the signed certificate
openssl x509 -req -in server-req.pem -days 356 -CA ca-cert.pem -CAkey ca-key.pem -CAcreateserial -out server-cert.pem -extfile server-ext.cnf;

# display certificate info
echo "Server's signed certificate";
openssl x509 -in server-cert.pem -noout -text;

# verify validity
openssl verify -CAfile ca-cert.pem server-cert.pem