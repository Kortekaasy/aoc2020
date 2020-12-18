extern crate pest;
extern crate pest_consume;
extern crate lazy_static;

use pest::prec_climber as pcl;
use pest::prec_climber::PrecClimber;

use pest_consume::Parser;
use pest_consume::Error;
use pest_consume::match_nodes;
// ========================= Challenge Logic ============================
#[derive(Parser)]
#[grammar = "../expr.pest"]
pub struct ExprParser;

type Result<T> = std::result::Result<T, Error<Rule>>;
type Node<'i> = pest_consume::Node<'i, Rule, ()>;

lazy_static::lazy_static! {
    static ref PRECCLIMBER: PrecClimber<Rule> = PrecClimber::new(
        vec![
            pcl::Operator::new(Rule::MUL, pcl::Assoc::Left), pcl::Operator::new(Rule::ADD, pcl::Assoc::Left),
        ]
    );
}

#[pest_consume::parser]
impl ExprParser {
    fn EOI(_input: Node) -> Result<()> {
        Ok(())
    }

    fn num(input: Node) -> Result<i64> {
        input.as_str().trim()
            .parse::<i64>()
            .map_err(|e| input.error(e))
    }

    #[prec_climb(val, PRECCLIMBER)]
    fn expr(l: i64, input: Node, r: i64) -> Result<i64> {
        use Rule::*;
        match input.as_rule() {
            ADD => Ok(l + r),
            MUL => Ok(l * r),
            r => Err(input.error(format!("Rule {:?} isn't an operator", r)))?
        }
    }

    fn val(input: Node) -> Result<i64> {
        Ok(match_nodes!(input.into_children();
            [num(n)] => n,
            [expr(e)] => e
        ))
    }

    fn calculation(input: Node) -> Result<i64> {
        Ok(match_nodes!(input.into_children();
            [expr(e).., EOI(_)] => e.sum(),
        ))
    }
}

pub fn parse_and_execute(input: String) -> String {
    let inputs = ExprParser::parse(Rule::calculation, input.as_str()).unwrap();
    let input = inputs.single().unwrap();
    let res = ExprParser::calculation(input);
    format!("{}", res.unwrap())
}