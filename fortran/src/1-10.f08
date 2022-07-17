module p1_10
   use index
   implicit none
contains
   function multiples_of_3_or_5() result(ret)
      use iso_fortran_env, only: int64
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
end module p1_10
