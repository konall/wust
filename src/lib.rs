use syn::__private::ToTokens;

// #[derive(Clone)]
// enum Value {
//     Lit(serde_json::Value),
//     Ref(String)
// }

// impl Default for Value {
//     fn default() -> Self {
//         Self::Lit(serde_json::Value::Null)
//     }
// }
use serde_json::Value;

#[derive(Clone, Default)]
struct Variable {
    mutable: bool,
    value: Value
}

#[derive(Clone, Default)]
struct Scope {
    locals: std::collections::BTreeMap<String, Variable>,
    value: Value
}

#[derive(Default)]
struct Program {
    functions: std::collections::BTreeMap<String, syn::ItemFn>,
    parent_scopes: Vec<Scope>,
    current_scope: Scope
}

impl<'ast> syn::visit::Visit<'ast> for Program {
    
    // -> misc
    
    // { ... }
    fn visit_block(&mut self, i: &'ast syn::Block) {
        self.parent_scopes.push(self.current_scope.clone());
        self.current_scope = Scope::default();
        
        syn::visit::visit_block(self, i);
        
        self.current_scope = self.parent_scopes.pop().unwrap();
    }
    
    // x!{}
    fn visit_macro(&mut self, i: &'ast syn::Macro) {
        todo!()//TODO: employ macro syntax as an API for a wust standard library?
    }
    
    
    // -> functions
    
    // fn gx(a:_, b:!) { ... }
    fn visit_item_fn(&mut self, i: &'ast syn::ItemFn) {
        self.functions.insert(i.sig.ident.to_string(), i.clone());
        return;
    }
    
    // gx(1, 2)
    fn visit_expr_call(&mut self, i: &'ast syn::ExprCall) {
        self.visit_expr(&*i.func);
        match &self.current_scope.value {
            // Value::Lit(literal) => {
                // match literal {
                    serde_json::Value::String(_) => todo!(),
                    _ => panic!()
                // }
            // },
            // Value::Ref(_) => todo!()
        }
        // if let Some(fx) = self.functions.get(&i.func)
    }
        
    // x.y()
    fn visit_expr_method_call(&mut self, i: &'ast syn::ExprMethodCall) {
        todo!()
    }
    
    // return x
    fn visit_expr_return(&mut self, i: &'ast syn::ExprReturn) {
        todo!()
    }
    
    
    // -> conditionals
    
    // if x { ... } else if y { ... } else { ... }
    fn visit_expr_if(&mut self, i: &'ast syn::ExprIf) {
        todo!()
    }
    
    // match x { y => { ...}, z => { ... } }
    fn visit_expr_match(&mut self, i: &'ast syn::ExprMatch) {
        todo!()
    }
    
    
    // -> loops
    
    // for i in x { ... }
    fn visit_expr_for_loop(&mut self, i: &'ast syn::ExprForLoop) {
        todo!()
    }
    
    // while x { ... }
    fn visit_expr_while(&mut self, i: &'ast syn::ExprWhile) {
        todo!()
    }
    
    // loop { ... }
    fn visit_expr_loop(&mut self, i: &'ast syn::ExprLoop) {
        todo!()
    }
    
    
    // -> assignments
    
    // let x = y();
    
    fn visit_local(&mut self, i: &'ast syn::Local) {
        if let syn::Pat::Ident(ident_pat) = &i.pat {
            let name = ident_pat.ident.to_string();
            // let value = {
            //     if let Some(local_init) = &i.init {
            //         self.visit_expr(&local_init.expr);
            //         self.scopes[&self.current_scope].value.clone()
            //     } else {
            //         Variable::default()
            //     }
            // };
            self.current_scope.locals.insert(name.clone(), Variable::default());
        } else {
            panic!("Assignment destructuring is not yet suppported -> {}", i.to_token_stream());
        }
    }
    
    // a = b();
    
    fn visit_expr_assign(&mut self, i: &'ast syn::ExprAssign) {
        match &*i.left {
            syn::Expr::Path(path_expr) => {
                if let Some(ident) = path_expr.path.get_ident() {
                    let variable = ident.to_string();
                    let mut scope = &self.current_scope;
                    if let Some(_) = {
                        loop {
                            if scope.locals.contains_key(&variable) {
                                break Some(());
                            }
                            // if let Some(parent_scope) = self.parent_scopes.get(i) {
                            //     scope = parent_scope;
                            // } else {
                            //     break None;
                            // }
                        }
                    } {
                        syn::visit::visit_expr_assign(self, i);
                    } else {
                        panic!("Cannot assign to undeclared variable '{variable}' -> {}", i.to_token_stream());
                    }
                } else {
                    panic!("Qualified paths are not yet supported -> {}", i.into_token_stream());
                }
            },
            syn::Expr::Field(_) => todo!(),
            syn::Expr::MethodCall(_) => todo!(),
            syn::Expr::Index(_) => todo!(),
            _ => panic!("Assignment destructuring is not yet supported -> {}", i.to_token_stream())
        }
    }
    
    
    // -> literals
    
    // x{ a: 1, b: 2 }
    fn visit_pat_struct(&mut self, i: &'ast syn::PatStruct) {
        let mut object = serde_json::json!({});
        for field in &i.fields {
            self.visit_member(&field.member);
            let key = self.current_scope.value.clone();
            self.visit_pat(&field.pat);//TODO: match only a strict subset of "pat"s here
            let value = self.current_scope.value.clone();
            object[key.as_str().unwrap()] = value;
        }
        // self.scopes.get_mut(&self.current_scope).unwrap().value = Value::Lit(object);
        self.current_scope.value = object;
    }
    
    // [a, "x", 1]
    fn visit_expr_array(&mut self, i: &'ast syn::ExprArray) {
        let mut array = serde_json::json!([]);
        for (i, el) in i.elems.iter().enumerate() {
            self.visit_expr(el);
            array[i] = self.current_scope.value.clone();
        }
        // self.scopes.get_mut(&self.current_scope).unwrap().value = Value::Lit(array);
        self.current_scope.value = array;
    }
    
    // "abc" / true / 123 / ()
    fn visit_lit(&mut self, i: &'ast syn::Lit) {
        todo!()
    }
    
    
    // -> access
    
    // x::y::z
    
    fn visit_path(&mut self, i: &'ast syn::Path) {
        if i.get_ident().is_none() {
            panic!("Qualified paths are not yet supported -> {}", i.into_token_stream());
        }
        
        syn::visit::visit_path(self, i);
    }
    
    // x.y.z
    fn visit_expr_field(&mut self, i: &'ast syn::ExprField) {
        todo!()
    }
    
    // x[y]
    fn visit_index(&mut self, i: &'ast syn::Index) {
        
    }
    
    // x
    fn visit_ident(&mut self, i: &'ast syn::Ident) {
        todo!()
    }
    
    
    // --- for future consideration ---
    
    // -> types
    
    // struct Abc { x: String, y: Number }
    fn visit_item_struct(&mut self, i: &'ast syn::ItemStruct) {
        todo!()
    }
    
    // enum Abc { X, Y, Z }
    fn visit_item_enum(&mut self, i: &'ast syn::ItemEnum) {
        todo!()
    }
    
    // type A = String
    fn visit_item_type(&mut self, i: &'ast syn::ItemType) {
        todo!()
    }
    
    
    // -> async
    
    // async { ... }
    fn visit_expr_async(&mut self, i: &'ast syn::ExprAsync) {
        todo!()
    }
    
    // x.await
    fn visit_expr_await(&mut self, i: &'ast syn::ExprAwait) {
        todo!()
    }
}

pub fn eval(src: &str) {
    use syn::visit::Visit;
    
    match syn::parse_file(src) {
        Ok(code) => Program::default().visit_file(&code),
        Err(err) => {
            match syn::parse_str::<syn::Expr>(src) {
                Ok(code) => Program::default().visit_expr(&code),
                _ => panic!("{err}")
            }
        }
    }
}

#[test]
fn fx() {
    eval("{
        let a = 1;
        let b = 2;
        a + b
    }");
}
