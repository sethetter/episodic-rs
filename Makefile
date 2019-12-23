PHONY := psql

psql:
	psql "host=0.0.0.0 port=5439 user=postgres password=super_secret_password dbname=episodic"
