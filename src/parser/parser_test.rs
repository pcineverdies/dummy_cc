mod test {

    #[test]
    fn parser_test_1() {
        use crate::lexer::lexer_impl::Lexer;
        use crate::parser::parser_impl::Parser;

        let input = String::from(
            "
            const u32 N = 30;

            void swap(u32* a, u32* b) {
              u32 temp = *a;
              *a = *b;
              *b = temp;
            }

            void bubble_sort(u32* array, u32 N) {
              u32 i;
              u32 j;
              for(i = 0; i < N; i = i + 1) {
                for(j = i; j < N; j = j + 1) {
                  if (array[j] < array[i]) {
                    swap(&array[i], &array[j]);
                  }
                }
              }
            }

            u32** experimental_function() {
              const u8 array[10];
              while(array[10] = 20){
                for(;;){}
                u32 daje = 80 * 20 - 32 << 21;
                if(daje){;;;;;}
              }
              return 0;
              break;
              continue;
            }

            void main() {
              u32 array[N];
              bubble_sort(array, N);
            }
        ",
        );

        let expected = String::from(
            "const u32 N = 30;
void swap(u32* a,u32* b){
  u32 temp = (*a);
  ((*a) = (*b));
  ((*b) = temp);
}
void bubble_sort(u32* array,u32 N){
  u32 i;
  u32 j;
  for((i = 0); (i < N); (i = (i + 1))){
    for((j = i); (j < N); (j = (j + 1))){
      if((((array)[j]) < ((array)[i]))){
        (swap)((&((array)[i])),(&((array)[j])));
      }
    }
  }
}
u32** experimental_function(){
  const u8 array[10];
  while((((array)[10]) = 20)){
    for(; ; ){}
    u32 daje = (((80 * 20) - 32) << 21);
    if(daje){}
  }
  return 0;
  break;
  continue;
}
void main(){
  u32 array[N];
  (bubble_sort)(array,N);
}
",
        );

        let mut l = Lexer::new(input.clone(), false).unwrap();
        let tokens = l.tokenize();
        if tokens.is_none() {
            assert!(false);
        }
        let mut p = Parser::new(tokens.unwrap(), String::from(""));
        let ast_wrapped = p.parse();
        if ast_wrapped.is_some() {
            assert_eq!(expected, ast_wrapped.clone().unwrap().to_string(0));
        } else {
            assert!(false);
        }
    }
}
