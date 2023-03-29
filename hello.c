
int main() {
  int list[] = {6, 5, 3, 1, 2, 4};
  int len = 6;
  for (int i in range(0, len - 1)) {
    for (int j in range(0, len - i - 1)) {
      if (list[j] > list[j + 1]) {
        int temp = list[j];
        list[j] = list[j + 1];
        list[j + 1] = temp;
      }
    }
  }
  println(list);
}
