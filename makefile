TEST_NAME=test
TEST_INTEGRATOR=verlet

TEST_ATOMS=300
TEST_BOXLENGTH=20
TEST_STEP=1e-3
TEST_TEMPERATURE=1

RUN_TEST_NEW=new $\
	--run-name ${TEST_NAME} $\
	--set-integrator ${TEST_INTEGRATOR} $\
	--set-atoms ${TEST_ATOMS} $\
	--set-boxlength ${TEST_BOXLENGTH} $\
	--set-step ${TEST_STEP} $\
	--set-temperature ${TEST_TEMPERATURE} $\

RUN_TEST_RESUME=resume $\
	--run-name ${TEST_NAME} $\
	--set-integrator ${TEST_INTEGRATOR}

build:
	cargo build --release

build-debug:
	cargo build

run-new-test-debug: build-debug
	time ./target/debug/molecular_dynamics ${RUN_TEST_NEW}

run-resume-test-debug: build-debug
	./target/debug/molecular_dynamics ${RUN_TEST_RESUME}

run-new-test-release: build
	time ./target/release/molecular_dynamics ${RUN_TEST_NEW}

run-resume-test-release: build
	./target/release/molecular_dynamics ${RUN_TEST_RESUME}

plot-test:
	if [[ -d result/${TEST_NAME} ]]; then
		gnuplot -p -e "filename='result/${TEST_NAME}/properties.csv';" result/energy.gnuplot
		gnuplot -p -e "filename='result/${TEST_NAME}/properties.csv';" result/pressure.gnuplot
		gnuplot -p -e "filename='result/${TEST_NAME}/properties.csv';" result/temperature.gnuplot
	fi
	
clean-test:
	rm -r result/${TEST_NAME}
