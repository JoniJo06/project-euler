module math
   implicit none
contains
   function pow(num) result(result)
      integer, intent(in) :: num
      integer :: result
      result = num * num
      ! integer, intent(out) :: result
   end function pow
end module math
