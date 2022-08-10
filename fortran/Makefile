FCFLAGS=-pedantic -Wall -Wextra
LIBS=
FC=gfortran

project-euler: src/main.f90
	$(FC) $(FCFLAGS) -o project-euler src/*.f90 $(LIBS)

.Phony: clean
clean:
	-rm -f *.o *.mod *.smod project-euler mod/*

