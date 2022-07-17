FCFLAGS=
LIBS=
FC=gfortran

project-euler: src/main.f08
	$(FC) $(FCFLAGS) -o project-euler src/*.f08 $(LIBS)

.Phony: clean
clean:
	-rm -f *.o *.mod *.smod project-euler mod/*

