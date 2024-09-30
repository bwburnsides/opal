```
            module => item*
              item => module
                    | func
                    | type-alias
                    | struct
                    | enum
                    | constant
                    | static
  
              func => FN IDENT LPAREN param* RPAREN (R_LARR type)? func-body
        type-alias => TYPE IDENT EQUAL type NEWLINE
            struct => STRUCT IDENT LBRACE (field (COMMA field)* COMMA?)? RBRACE
              enum => ENUM IDENT LBRACE (variant (COMMA variant)* COMMA?)? RBRACE
          constant => CONST IDENT COLON type EQUAL expr NEWLINE
            static => STATIC IDENT COLON type EQUAL expr NEWLINE
 
              type => builtin-type
                    | LBRACK type SEMICOLON INTEGER_LITERAL RBRACK
                    | AMPER MUT? type
                    | LPAREN type RPAREN
                    | COLON2? IDENT (COLON2 IDENT)*

             param => MUT? IDENT COLON type
             field => IDENT COLON type
 
         func-body => block-expr | NEWLINE
              stmt => item
                    | let-stmt
                    | expr-stmt
                    | continue-stmt
                    | break-stmt
 
          let-stmt => LET IDENT COLON type EQUAL expr NEWLINE
         expr-stmt => expr-without-block NEWLINE
                    | expr-with-block NEWLINE?
     continue-stmt => CONTINUE NEWLINE
        break-stmt => BREAK NEWLINE

              expr => expr-without-block
                    | expr-with-block

expr-without-block => literal-expr
                    | path-expr 
                    | operator-expr
                    | grouped-expr
                    | array-expr
                    | index-expr
                    | call-expr
                    | field-expr
                    | return-expr

   expr-with-block => block-expr
                    | if-expr
                    | when-expr
                    | loop-expr

      literal-expr => CHAR_LITERAL
                    | STRING_LITERAL
                    | INTEGER_LITERAL
                    | FLOAT_LITERAL
                    | TRUE
                    | FALSE
         path-expr => COLON2? IDENT (COLON2 IDENT)*
     operator-expr => AMPER MUT? expr
                    | ASTERISK expr
                    | expr QUESTION
                    | HYPHEN expr
                    | EXCLAM expr
                    | expr PLUS expr
                    | expr HYPHEN expr
                    | expr ASTERISK expr
                    | expr FSLASH expr
                    | expr AMPER expr
                    | expr BAR expr
                    | expr LANGLE2 expr
                    | expr RANGLE2 expr
                    | expr EQUAL2 expr
                    | expr EXCLAM_EQUAL expr
                    | expr RANGLE expr
                    | expr LANGLE expr
                    | expr RANGLE_EQUAL expr
                    | expr LANGLE_EQUAL expr
                    | expr BAR2 expr
                    | expr AMPER2 expr
                    | expr EQUAL expr
                    | expr PLUS_EQUAL expr
                    | expr HYPHEN_EQUAL expr
                    | expr ASTERISK_EQUAL expr
                    | expr FSLASH_EQUAL expr
                    | expr AMPER_EQUAL expr
                    | expr BAR_EQUAL expr
                    | expr LANGLE2_EQUAL expr
                    | expr RANGLE2_EQUAL expr
      grouped-expr => LPAREN expr RPAREN
        array-expr => LBRACK expr-list? RBRACK
        index-expr => expr LBRACK expr RBRACK
         call-expr => expr LPAREN expr-list? RPAREN
        field-expr => expr PERIOD IDENT
       return-expr => RETURN expr?

        block-expr => LBRACE stmt* RBRACE
           if-expr => IF expr block-expr else-clause?
         when-expr => WHEN expr LBRACE when-arms? RBRACE
         loop-expr => FOR IDENT IN expr block-expr
                    | WHILE expr block-expr

         expr-list => expr (COMMA expr)* COMMA?
       else-clause => ELSE (block-expr | if-expr)
         when-arms => (when-arm R_HARR expr-with-block)*
          when-arm => IS type (IF expr)?
```