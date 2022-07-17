program main
   use p1_10
   use string
   use index
   implicit none



   integer :: stat
   integer :: argc
   character(len=10) :: arg
   character(len=32) :: program
   integer :: problem
   type(tuple) :: result

   argc = command_argument_count()
   call get_command_argument(0, program)
   if ( argc < 1 ) then
      print '("Usage: ", g0, " <problem-number>")', trim(program)
      stop
   end if
   call get_command_argument(1, arg)
   call str2int(arg, problem, stat)
   if ( stat /= 0 ) then
      print '("Conversion of string failed!")'
      call exit(1)
   end if

   select case (problem)
    case (1)
      result = multiples_of_3_or_5()
    case default
      print '("Unknown problem number: ", i0)', problem
      stop
   end select

   print '(a)', '----------------------------------------'
   print '(g0)', result%name
   print '("Result: ", I0)', result%result
   print '(a)', '----------------------------------------'

   call compare(problem, result%result)

end program main
