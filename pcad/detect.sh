while true
do
    b=$(./detect)
    if [ $b -ne 0 ]
    then
        echo input is $b
        break
    fi
done