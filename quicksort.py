from array import array

# quicksorting by recrusion
def quickSorting(unsortedArray):
    #base case if there is zero or one index
    if len(unsortedArray) < 2:
        return unsortedArray
    else:
        pivot = unsortedArray[0]
        #array of index less than the pivot value
        smaller = [i for i in unsortedArray[1:] if i <= pivot]
        #array of index more than the pivot value
        larger = [i for i in unsortedArray[1:] if i > pivot]
        
        return quickSorting(smaller) + [pivot] + quickSorting(larger)

print quickSorting([1, 5, 6, 8, 9, 10, 2, 3])