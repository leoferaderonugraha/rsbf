pub trait Expression {
    fn accept(&self, visitor: &mut dyn Visitor);
}

pub struct MoveRightExpression;
pub struct MoveLeftExpression;
pub struct AddExpression;
pub struct SubExpression;
pub struct DotExpression;
pub struct CommaExpression;
pub struct OpenSquareExpression;
pub struct CloseSquareExpression;

pub struct ExpressionVisitor;

pub trait Visitor {
    /** These Expressions (expr param) becomes obsolete **/
    fn visit_move_right(&mut self, expr: &MoveRightExpression);
    fn visit_move_left(&mut self, expr: &MoveLeftExpression);
    fn visit_add(&mut self, expr: &AddExpression);
    fn visit_sub(&mut self, expr: &SubExpression);
    fn visit_dot(&mut self, expr: &DotExpression);
    fn visit_comma(&mut self, expr: &CommaExpression);
    fn visit_open_square(&mut self, expr: &OpenSquareExpression);
    fn visit_close_square(&mut self, expr: &CloseSquareExpression);
}

impl Expression for MoveRightExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_move_right(self);
    }
}

impl Expression for MoveLeftExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_move_left(self);
    }
}

impl Expression for AddExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_add(self);
    }
}

impl Expression for SubExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_sub(self);
    }
}

impl Expression for DotExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_dot(self);
    }
}

impl Expression for CommaExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_comma(self);
    }
}

impl Expression for OpenSquareExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_open_square(self);
    }
}

impl Expression for CloseSquareExpression {
    fn accept(&self, visitor: &mut dyn Visitor) {
        visitor.visit_close_square(self);
    }
}
