module p1_10
   use index
   use string
   use iso_fortran_env, only: int64, int8
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

      ret%name = "1# multiples of 3 or 5"
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

      ret%name = "2# even fibonacci numbers"
      ret%result = result

   end function even_fibonacci_numbers

   function largest_prime_factor() result(ret)
      implicit none
      type(tuple) :: ret
      integer(kind=int64) :: result = 0
      integer(kind=8) :: root_number = 600851475143_8
      integer(kind=8) :: i

      do while (mod(root_number, 2_8).eq.0)
         root_number = root_number / 2_8
      end do

      do i=3, int(sqrt(real(root_number)))
         do while (mod(root_number, i).eq.0)
            result = i
            root_number = root_number / i
         end do
      end do

      ret%name = "3# largest prime factor"
      ret%result = result
   end function largest_prime_factor

   function largest_palindrome_product() result(ret)
      implicit none
      type(tuple) :: ret
      integer(kind=int64) :: result = 0
      integer :: i
      integer :: j
      integer :: product
      character(len=10) :: product_string
      integer :: reverse_product
      integer :: str2int_stat

      do i = 100, 999
         do j = 100, 999
            product = i * j
            write(product_string, "(i0)") product
            call str2int(reverse(product_string), reverse_product, str2int_stat)
            if (reverse_product == product .and. product > result) then
               result = product
            end if
         end do
      end do

      ret%name = "4# largest palindrome product"
      ret%result = result
   end function largest_palindrome_product

   function smallest_multiple() result(ret)
      implicit none
      type(tuple) :: ret
      integer(kind=int64) :: result = 0
      integer :: n = 20
      logical :: dividable = .true.
      integer :: i

      do while (.true.)
         dividable = .true.
         do i = 1, 20
            if(mod(n, i).ne.0) then
               dividable = .false.
            end if
         end do
         if (dividable) then
            result = n
            exit
         end if
         n = n + 20
      end do

      ret%name = "5# smallest multiple"
      ret%result = result
   end function smallest_multiple
end module p1_10
