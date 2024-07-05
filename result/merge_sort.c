// Code from: https://www.geeksforgeeks.org/merge-sort/

void merge(int* array, int left, int mid, int right){
  int subArrayOne = mid - left + 1;
  int subArrayTwo = right - mid;

  int i;
  int j;

  int leftArray[subArrayOne];
  int rightArray[subArrayTwo];

  for (i = 0; i < subArrayOne; i = i + 1){
    leftArray[i] = array[left + i];
  }
  for (j = 0; j < subArrayTwo; j = j + 1){
    rightArray[j] = array[mid + 1 + j];
  }

  int indexOfSubArrayOne = 0;
  int indexOfSubArrayTwo = 0;
  int indexOfMergedArray = left;

  while ((indexOfSubArrayOne < subArrayOne) & (indexOfSubArrayTwo < subArrayTwo)) {
    if (leftArray[indexOfSubArrayOne] <= rightArray[indexOfSubArrayTwo]) {
      array[indexOfMergedArray] = leftArray[indexOfSubArrayOne];
      indexOfSubArrayOne = indexOfSubArrayOne + 1;
    }
    else {
      array[indexOfMergedArray] = rightArray[indexOfSubArrayTwo];
      indexOfSubArrayTwo = indexOfSubArrayTwo + 1;
    }
    indexOfMergedArray = indexOfMergedArray + 1;
  }

  while (indexOfSubArrayOne < subArrayOne) {
    array[indexOfMergedArray] = leftArray[indexOfSubArrayOne];
    indexOfSubArrayOne = indexOfSubArrayOne + 1;
    indexOfMergedArray = indexOfMergedArray + 1;
  }

  while (indexOfSubArrayTwo < subArrayTwo) {
    array[indexOfMergedArray] = rightArray[indexOfSubArrayTwo];
    indexOfSubArrayTwo = indexOfSubArrayTwo + 1;
    indexOfMergedArray = indexOfMergedArray + 1;
  }

  return;
}

void mergeSort(int* array, int begin, int end){
  if (begin >= end){
    return;
  }

  int mid = begin + ((end - begin) >> 1);
  mergeSort(array, begin, mid);
  mergeSort(array, mid + 1, end);
  merge(array, begin, mid, end);

  return;
}


int main(){
  int arr_size = 10;
  int arr[arr_size];
  int i;
  for(i = 0; i < arr_size; i = i + 1) {
    arr[arr_size - i - 1] = (2 << i);
  }

  mergeSort(arr, 0, arr_size - 1);

  return 0;
}
