KEYPAIR_FILE = $(shell solana config get | grep "Keypair Path" | awk '{print $$3}')
PROGRAM_KEYPAIR_FILE = keypair.json
PROGRAM_SO = ./target/deploy/program.so

.PHONY: clean build deploy ${PROGRAM_SO}

get_pubkey = $(shell solana-keygen pubkey ${PROGRAM_KEYPAIR_FILE})

$(PROGRAM_KEYPAIR_FILE):
	@echo "Generating keypair..."
	@solana-keygen new --outfile $(PROGRAM_KEYPAIR_FILE) --no-bip39-passphrase 
	@echo "Done."

clean:
	@echo "Cleaning..."
	@cargo clean
	@echo "Done."

${PROGRAM_SO}: 
	@echo "Building..."
	@cargo build-bpf
	@echo "Done."

build: ${PROGRAM_SO} ${PROGRAM_KEYPAIR_FILE}
	@echo "Building..."
	@cargo build-bpf
	@echo "Done."

test: build
	@echo "Testing..."
	@env PROGRAM_ID=$(call get_pubkey) \
		PAYER_KEYPAIR_PATH=${KEYPAIR_FILE} \
		cargo test -- --nocapture
	@echo "Done."

deploy: build
	@echo "Deploying..."
	solana program deploy ./target/deploy/program.so \
		--upgrade-authority "${PROGRAM_KEYPAIR_FILE}" \
		--program-id "$(shell solana address -k ${PROGRAM_KEYPAIR_FILE})"
	@echo "Done."

