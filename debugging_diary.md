# Debugging Diary

## Add Order Error Display Bug
On the add order screen the error messages are not meant to be displayed
until the user interacts with the corrosponding input box. A bug I found 
was that the error messages for the return on date input would trigger 
after the hired on input was interacted with. After debugging I found that 
the problem lied in the method that checked to see if the error message 
for a given input should be shown. I found that when you gave it the name 
for the return on input, it would check the hired on variable instead of 
the return on variable. After fixing that the bug was gone.

## Database Storage Directory Bug
I found that the orders in the database would change based on what
directory I was in. The bug turned out to be caused by the fact that the
database was stored in the directory you were running the app from, so
if you changed directory, a new database was created. I fixed this by
storing the database in a user-specific OS-specific data directory.
After this you would use the same database no matter where you ran the 
program from.
