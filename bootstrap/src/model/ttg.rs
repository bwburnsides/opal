pub trait TreeData<Phase> {
    type Case;
    type IfIs;
    type For;
    type ErrorPropagation;
    type Return;
    type Break;
    type Continue;
    type Block;
    type Grouped;
    type Path;
    type Literal;
    type Array;
    type Prefix;
    type Binary;
    type Call;
    type Field;
    type Index;
    type Other;

    type NameRepresentation;
    type PathRepresentation;
    type TypeRepresentation;
}