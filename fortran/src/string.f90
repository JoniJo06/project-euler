module string
contains
   elemental subroutine str2int(str,int,stat)
      implicit none
      character(len=*),intent(in) :: str
      integer,intent(out)         :: int
      integer,intent(out)         :: stat

      read(str,*,iostat=stat)  int
   end subroutine str2int

   pure function reverse(string) result(reverse_string)
      character(len=*), intent(in) :: string
      character(len=len(string)) :: reverse_string
      integer :: i, n

      n = len(string)
      do i = 1, n
         reverse_string(n-i+1:n-i+1) = string(i:i)
      end do

   end function reverse
end module string
