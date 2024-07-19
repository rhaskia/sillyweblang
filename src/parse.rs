pub fn parse(program: Vec<Sp<Token>) -> AstNode {
    Parser { program }.parse()
}

struct Parser {
    program: Vec<Sp<Token>>,
    index: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Result<Sp<Node>, Error> {
        self.expr()
    }

    pub fn expr(&mut self) -> Result<Sp<Node>, Error> {
        let left = self.call()?;

        if Self::is_op(&self.peek().inner) {
            let op = self.next().inner.clone();
            let right = self.expr()?;

            // ordering switching
            if let Node::Binary { left: ref r_left, op: ref r_op, right: ref r_right } = *right {
                if Self::op_order(&op) > Self::op_order(r_op) {
                    return Sp<Node>::new_binary(
                        Sp<Node>::new_binary(left, op, *r_left.clone()),
                        r_op.clone(),
                        *r_right.clone(),
                    );
                }
            }

            return Sp<Node>::new_binary(left, op, right);
        }

        left
    }
    
    pub fn op_order(operator: &Operator) {
        match operator {

        }
    }

    pub fn next() -> Token {
        self.index += 1;
        self.tokens[index - 1].clone()
    }
}
