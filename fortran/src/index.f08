module index
   use iso_fortran_env, only: int64
   implicit none
   type :: tuple
      character(len=:), allocatable :: name
      integer(kind=int64), allocatable :: result
   end type tuple

contains
   subroutine compare(problem, result)
      use string
      use iso_fortran_env, only: int64
      implicit none
      integer, intent(in) :: problem
      integer(kind=int64), intent(in) :: result
      integer, parameter :: read_unit = 99
      character(len=200), allocatable :: command(:)
      character(len=200) :: line
      integer :: ios
      integer :: n, i
      integer :: n_command
      integer :: string_stat

      open(unit=read_unit, file='./../results.txt', iostat=ios)
      if ( ios /= 0 ) stop "Error opening file results.txt"

      n = 0

      do
         read(read_unit, '(A)', iostat=ios) line
         if (ios /= 0) exit
         n = n + 1
      end do

      allocate(command(n))

      rewind(read_unit)

      do i = 1, n
         read(read_unit, '(A)') command(i)
      end do

      close(read_unit)

      do i = 1, n
         call str2int(command(i), n_command, string_stat)
         if (i == problem) then
            if (result /= n_command) then
               print '("Error: ", i0, " != ", i0)', n_command, result
               call exit(1)
            end if
         end if
      end do
   end subroutine compare
end module index
