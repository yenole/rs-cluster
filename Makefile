CG ?= cargo

PYHON: binary
binary: dist
	$(CG) version
	$(CG) build --release

dist:
	mkdir $@

dev: dist
	$(CG) watch -w ./src/ -c -x 'run' 

deploy: binary
	scp ./target/release/rs-cluster ubuntu@rise.shub.gq:/data/cluster/app2
	# $(CG) clean
