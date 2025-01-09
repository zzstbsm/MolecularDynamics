RUN_TEST_NAME=test
RUN_TEST_INTEGRATOR=verlet

RUN_TEST_ATOMS=300
RUN_TEST_BOXLENGTH=20
RUN_TEST_STEP=1e-3
RUN_TEST_TEMPERATURE=1

build:
	cargo build

new-test:
	cargo run -r -- new \
		--run-name ${RUN_TEST_NAME} \
		--set-integrator ${RUN_TEST_INTEGRATOR} \
		--set-atoms ${RUN_TEST_ATOMS} \
		--set-boxlength ${RUN_TEST_BOXLENGTH} \
		--set-step ${RUN_TEST_STEP} \
		--set-temperature ${RUN_TEST_TEMPERATURE}

resume-test:
	cargo run -r -- resume \
		--run-name ${RUN_TEST_NAME} \
		--set-integrator ${RUN_TEST_INTEGRATOR}

plot-test:
	gnuplot -p -e "filename='result/${RUN_TEST_NAME}/properties.csv';" result/energy.gnuplot
	gnuplot -p -e "filename='result/${RUN_TEST_NAME}/properties.csv';" result/pressure.gnuplot
	gnuplot -p -e "filename='result/${RUN_TEST_NAME}/properties.csv';" result/temperature.gnuplot

clean-test:
	rm -r result/${RUN_TEST_NAME}
