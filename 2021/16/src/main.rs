#[derive(Debug, Clone)]
enum Packet {
    Literal {
        version: u8,
        value: u64,
    },
    Operator {
        version: u8,
        type_id: u8,
        children: Vec<Packet>,
    },
}

impl Packet {
    fn parse_literal_value(bits: &mut impl Iterator<Item = u8>) -> u64 {
        let mut val = 0;
        loop {
            let prefix = bits.next().unwrap();
            val <<= 4;
            val += bits
                .take(4)
                .map(|n| n as u64)
                .reduce(|acc, item| (acc << 1) + item)
                .unwrap() as u64;
            if prefix == 0 {
                return val;
            }
        }
    }

    fn parse_operator_children(bits: &mut impl Iterator<Item = u8>) -> Vec<Self> {
        let length_type = bits.next().unwrap();

        if length_type == 0 {
            let total_length = bits
                .take(15)
                .map(|n| n as usize)
                .reduce(|acc, item| (acc << 1) + item)
                .unwrap();
            let mut subpackets = Vec::new();
            let mut cut_iter = bits
                .take(total_length)
                .collect::<Vec<_>>()
                .into_iter()
                .peekable();
            while cut_iter.peek().is_some() {
                subpackets.push(Self::parse(&mut cut_iter))
            }
            subpackets
        } else {
            let subpacket_count = bits
                .take(11)
                .map(|n| n as u64)
                .reduce(|acc, item| (acc << 1) + item)
                .unwrap();
            (0..subpacket_count).map(|_| Self::parse(bits)).collect()
        }
    }

    fn parse(bits: &mut impl Iterator<Item = u8>) -> Packet {
        let version = bits
            .by_ref()
            .take(3)
            .reduce(|acc, item| (acc << 1) + item)
            .unwrap();
        let type_id = bits
            .by_ref()
            .take(3)
            .reduce(|acc, item| (acc << 1) + item)
            .unwrap();

        if type_id == 4 {
            Packet::Literal {
                version,
                value: Self::parse_literal_value(bits),
            }
        } else {
            Packet::Operator {
                version,
                type_id,
                children: Self::parse_operator_children(bits),
            }
        }
    }

    fn version_sum(&self) -> u64 {
        match self {
            Packet::Literal { version, .. } => *version as u64,
            Packet::Operator {
                version, children, ..
            } => *version as u64 + (*children).iter().map(Self::version_sum).sum::<u64>(),
        }
    }

    fn eval(&self) -> u64 {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator {
                type_id: 0,
                children,
                ..
            } => children.iter().map(Self::eval).sum(),
            Packet::Operator {
                type_id: 1,
                children,
                ..
            } => children.iter().map(Self::eval).product(),
            Packet::Operator {
                type_id: 2,
                children,
                ..
            } => children.iter().map(Self::eval).min().unwrap(),
            Packet::Operator {
                type_id: 3,
                children,
                ..
            } => children.iter().map(Self::eval).max().unwrap(),
            Packet::Operator {
                type_id: 5,
                children,
                ..
            } => (children[0].eval() > children[1].eval()) as u64,
            Packet::Operator {
                type_id: 6,
                children,
                ..
            } => (children[0].eval() < children[1].eval()) as u64,
            Packet::Operator {
                type_id: 7,
                children,
                ..
            } => (children[0].eval() == children[1].eval()) as u64,
            _ => panic!("Unknown type_id for operator"),
        }
    }
}

fn main() {
    // let input = "D2FE28";
    // let input = "38006F45291200";
    // let input = "EE00D40C823060";
    // let input = "8A004A801A8002F478";
    // let input = "A0016C880162017C3686B18A3D4780";
    let input = include_str!("../input.txt").strip_suffix('\n').unwrap();
    let mut bits = input.chars().flat_map(|c| {
        let n = c.to_digit(16).unwrap() as u8;
        [n >> 3, n >> 2 & 0b1, n >> 1 & 0b1, n & 0b1]
    });

    let packet = Packet::parse(&mut bits);
    println!("Version sum: {}", packet.version_sum());
    println!("Eval: {}", packet.eval());
}
