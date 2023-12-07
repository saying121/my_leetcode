##start#
# Read from the file file.txt and output the tenth line to stdout.
##start#

# head -n 10 file.txt | tail -n 1
awk 'NR==10' file.txt
