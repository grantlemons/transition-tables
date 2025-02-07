use std::error::Error;

/// A transition in the DFA transition table
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum TransitionTableTransition {
    /// A transition to a state
    Ok(usize),

    /// A transition to an error state
    Err,
}

/// The starting state ID
pub const STARTING_STATE_ID: usize = 0;

/// A state (row) in the DFA transition table
#[derive(Clone, Debug, PartialEq)]
pub struct TransitionTableRow {
    /// Whether the row is for an accepting state (+) or not (-)
    pub accepting: bool,

    /// The row's state ID (0 means the starting state)
    pub id: usize,

    /// The row's state transitions
    pub transitions: Vec<TransitionTableTransition>,
}

/// A DFA transition table
#[derive(Clone, Debug, PartialEq)]
pub struct TransitionTable {
    /// The rows in the table, sorted by state ID
    pub rows: Vec<TransitionTableRow>,
}

/// Errors that can occur when parsing or serializing a transition table
#[derive(Debug)]
pub struct ParseSerializeError {
    /// The error message
    pub message: String,
}

impl std::fmt::Display for ParseSerializeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

/// The symbol for an error transition
const ERROR_SYMBOL: &str = "E";

impl TransitionTable {
    /// Parse a transition table from a string
    pub fn parse(input: &str) -> Result<Self, ParseSerializeError> {
        let mut table = TransitionTable { rows: Vec::new() };
        let mut expected_columns: Option<usize> = None;

        // Split the input into lines
        for (line_index, line) in input.lines().enumerate() {
            let mut row = TransitionTableRow {
                accepting: false,
                id: 0,
                transitions: Vec::new(),
            };

            // Split the line into columns
            let columns = line.split_whitespace().collect::<Vec<_>>();

            // Check that there are at least two columns
            if columns.len() < 2 {
                return Err(ParseSerializeError {
                    message: format!("Line {} has too few columns", line_index + 1),
                });
            }

            // Check that the number of columns is consistent
            match expected_columns {
                Some(expected) => {
                    if expected != columns.len() {
                        return Err(ParseSerializeError {
                            message: format!(
                                "Line {} has a different number of columns than the previous lines",
                                line_index + 1
                            ),
                        });
                    }
                }
                None => {
                    expected_columns = Some(columns.len());
                }
            }

            // Parse accepting state column
            match columns[0].chars().next().unwrap() {
                '+' => {
                    row.accepting = true;
                }
                '-' => {
                    row.accepting = false;
                }
                _ => {
                    return Err(ParseSerializeError {
                        message: format!("Line {} has an invalid accepting state", line_index + 1),
                    });
                }
            }

            // Parse the ID column
            row.id = columns[1].parse().map_err(|e| ParseSerializeError {
                message: format!("Line {} has an invalid state ID: {}", line_index + 1, e),
            })?;

            // Parse the transitions
            for (column_index, column) in columns.iter().skip(2).enumerate() {
                // Parse the transition
                if *column == ERROR_SYMBOL {
                    row.transitions.push(TransitionTableTransition::Err);
                } else {
                    row.transitions
                        .push(TransitionTableTransition::Ok(column.parse().map_err(
                            |e| ParseSerializeError {
                                message: format!(
                                    "Line {} column {} has an invalid transition: {}",
                                    line_index + 1,
                                    column_index + 3,
                                    e
                                ),
                            },
                        )?))
                }
            }

            // Add the row to the table
            table.rows.push(row);
        }

        // Sort the rows by state ID
        table.rows.sort_by_key(|row| row.id);

        Ok(table)
    }

    /// Serialize the transition table to a string
    pub fn serialize(&self) -> Result<String, ParseSerializeError> {
        let mut output = String::new();

        for (row_index, row) in self.rows.iter().enumerate() {
            // Write the accepting state
            output.push(if row.accepting { '+' } else { '-' });
            output.push(' ');

            // Write the state ID
            output.push_str(&row.id.to_string());

            // Write the transitions
            for transition in &row.transitions {
                match transition {
                    TransitionTableTransition::Ok(state) => {
                        output.push(' ');
                        output.push_str(&state.to_string());
                    }
                    TransitionTableTransition::Err => {
                        output.push(' ');
                        output.push_str(ERROR_SYMBOL);
                    }
                }
            }

            if row_index < self.rows.len() - 1 {
                output.push('\n');
            }
        }

        Ok(output)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The provided transition table
    const PROVIDED_TRANSITION_TABLE: &str = r#"- 0 1 E E E E
- 1 E 2 E E E
- 2 2 3 2 2 2
- 3 4 3 2 2 2
+ 4 E E E E E"#;

    #[test]
    fn transition_table_parse() -> Result<(), ParseSerializeError> {
        let table = TransitionTable::parse(PROVIDED_TRANSITION_TABLE)?;

        assert_eq!(table.rows.len(), 5);

        assert_eq!(table.rows[0].accepting, false);
        assert_eq!(table.rows[0].id, 0);
        assert_eq!(table.rows[0].transitions.len(), 5);
        assert_eq!(
            table.rows[0].transitions[0],
            TransitionTableTransition::Ok(1)
        );
        assert_eq!(table.rows[0].transitions[1], TransitionTableTransition::Err);
        assert_eq!(table.rows[0].transitions[2], TransitionTableTransition::Err);
        assert_eq!(table.rows[0].transitions[3], TransitionTableTransition::Err);
        assert_eq!(table.rows[0].transitions[4], TransitionTableTransition::Err);

        assert_eq!(table.rows[1].accepting, false);
        assert_eq!(table.rows[1].id, 1);
        assert_eq!(table.rows[1].transitions.len(), 5);
        assert_eq!(table.rows[1].transitions[0], TransitionTableTransition::Err);
        assert_eq!(
            table.rows[1].transitions[1],
            TransitionTableTransition::Ok(2)
        );
        assert_eq!(table.rows[1].transitions[2], TransitionTableTransition::Err);
        assert_eq!(table.rows[1].transitions[3], TransitionTableTransition::Err);
        assert_eq!(table.rows[1].transitions[4], TransitionTableTransition::Err);

        assert_eq!(table.rows[2].accepting, false);
        assert_eq!(table.rows[2].id, 2);
        assert_eq!(table.rows[2].transitions.len(), 5);
        assert_eq!(
            table.rows[2].transitions[0],
            TransitionTableTransition::Ok(2)
        );
        assert_eq!(
            table.rows[2].transitions[1],
            TransitionTableTransition::Ok(3)
        );
        assert_eq!(
            table.rows[2].transitions[2],
            TransitionTableTransition::Ok(2)
        );
        assert_eq!(
            table.rows[2].transitions[3],
            TransitionTableTransition::Ok(2)
        );
        assert_eq!(
            table.rows[2].transitions[4],
            TransitionTableTransition::Ok(2)
        );

        assert_eq!(table.rows[3].accepting, false);
        assert_eq!(table.rows[3].id, 3);
        assert_eq!(table.rows[3].transitions.len(), 5);
        assert_eq!(
            table.rows[3].transitions[0],
            TransitionTableTransition::Ok(4)
        );
        assert_eq!(
            table.rows[3].transitions[1],
            TransitionTableTransition::Ok(3)
        );
        assert_eq!(
            table.rows[3].transitions[2],
            TransitionTableTransition::Ok(2)
        );
        assert_eq!(
            table.rows[3].transitions[3],
            TransitionTableTransition::Ok(2)
        );
        assert_eq!(
            table.rows[3].transitions[4],
            TransitionTableTransition::Ok(2)
        );

        assert_eq!(table.rows[4].accepting, true);
        assert_eq!(table.rows[4].id, 4);
        assert_eq!(table.rows[4].transitions.len(), 5);
        assert_eq!(table.rows[4].transitions[0], TransitionTableTransition::Err);
        assert_eq!(table.rows[4].transitions[1], TransitionTableTransition::Err);
        assert_eq!(table.rows[4].transitions[2], TransitionTableTransition::Err);
        assert_eq!(table.rows[4].transitions[3], TransitionTableTransition::Err);
        assert_eq!(table.rows[4].transitions[4], TransitionTableTransition::Err);

        Ok(())
    }

    #[test]
    fn transition_table_serialize() -> Result<(), ParseSerializeError> {
        let input = TransitionTable {
            rows: vec![
                TransitionTableRow {
                    accepting: false,
                    id: 0,
                    transitions: vec![
                        TransitionTableTransition::Ok(1),
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                    ],
                },
                TransitionTableRow {
                    accepting: false,
                    id: 1,
                    transitions: vec![
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Ok(2),
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                    ],
                },
                TransitionTableRow {
                    accepting: false,
                    id: 2,
                    transitions: vec![
                        TransitionTableTransition::Ok(2),
                        TransitionTableTransition::Ok(3),
                        TransitionTableTransition::Ok(2),
                        TransitionTableTransition::Ok(2),
                        TransitionTableTransition::Ok(2),
                    ],
                },
                TransitionTableRow {
                    accepting: false,
                    id: 3,
                    transitions: vec![
                        TransitionTableTransition::Ok(4),
                        TransitionTableTransition::Ok(3),
                        TransitionTableTransition::Ok(2),
                        TransitionTableTransition::Ok(2),
                        TransitionTableTransition::Ok(2),
                    ],
                },
                TransitionTableRow {
                    accepting: true,
                    id: 4,
                    transitions: vec![
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                        TransitionTableTransition::Err,
                    ],
                },
            ],
        };

        let output = input.serialize()?;

        assert_eq!(output, PROVIDED_TRANSITION_TABLE);

        Ok(())
    }
}
