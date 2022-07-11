import { exit } from "process";
import multiples_of_3_or_5 from "./1-10/1#_multiples_of_3_or_5";

const DEBUG = false;

const main = () => {
  let argv = process.argv;
  // remove first irrelevant argument
  argv.shift();
  const program: string = argv.shift() || exit(1);

  if (argv.length < 1) {
    console.error(`Usage: ${program} <problem number>`);
    exit(1);
  }

  if (DEBUG)
    console.log(argv)

  // @ts-ignore-next-line
  const problem: number = parseInt(argv.shift(), 10);
  if (isNaN(problem)) {
    console.error(`Usage: ${program} <problem number>`);
		exit(1);
  }

  switch (problem) {
    case 1: {
      console.log(multiples_of_3_or_5())
      break;
    }
  }

  if (DEBUG)
    console.log(problem);
} 
main();