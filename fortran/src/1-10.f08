module p1_10
   use index
   use iso_fortran_env, only: int64
   implicit none
contains
   function multiples_of_3_or_5() result(ret)
      implicit none
      type(tuple) :: ret
      integer :: n
      integer(kind=int64) :: result = 0

      do n = 0, 999
         if (mod(n, 3).eq.0 .or. mod(n, 5).eq.0) then
            result = result + n
         end if
      end do

      ret%name = "multiples of 3 or 5"
      ret%result = result

   end function multiples_of_3_or_5

   function even_fibonacci_numbers() result(ret)
      implicit none
      type(tuple) :: ret
      integer(kind=int64) :: result = 0
      integer :: a = 1
      integer :: b = 2
      integer :: temp

      do while (b.le.4000000)
         if (mod(b, 2).eq.0) then
            result = result + b
         end if
         temp = a + b
         a = b
         b = temp
      end do

      ret%name = "even fibonacci numbers"
      ret%result = result

   end function even_fibonacci_numbers
end module p1_10
