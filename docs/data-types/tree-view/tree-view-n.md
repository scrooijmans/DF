Data Structures in Typescript: Traversable N-ary Tree (part 1)
T.L.;D.R.: Learn how to build a Tree that can be navigated up and down easily. It works for nested menus, folder structures, and even routes.
Alexandre Lopes
Alexandre Lopes

Follow
4 min read
·
Sep 9, 2022
5


1





Press enter or click to view image in full size

If you’re unfamiliar with the tree data structure, it’s a graph where each node — except the root — must have a parent node. And that graph’s shape must be open: no parent/children relationship is allowed to be circular. In its simplest form, it should look like this:


const root = { parent: null }; 
const child_A = { parent: root }; 
const child_AA = { parent: child_A }; 
const child_AB = { parent: child_A }; 
const child_AC = { parent: child_A }; 
const child_B = { parent: root };
This is an N-ary tree because each parent can have N children. And as we’re declaring its nodes so many times, we may as well abstract it into a class:

export class TreeNode 
{ 
  public parent: TreeNode|null;

  constructor(parent: TreeNode|null) 
  { 
    this.parent = parent; 
  } 
}

const root = new TreeNode(null); 
const child_A = new TreeNode(root); 
const child_AA = new TreeNode(child_A); 
const child_AB = new TreeNode(child_A); 
const child_AC = new TreeNode(child_A); 
const child_B = new TreeNode(root);
Each node knows its parent. But they’ve been separated at birth as no parent knows their children. That’s super sad, so we’re about to fix that.

Letting the parents know their children in two lines
export class TreeNode 
{
  public parent: TreeNode|null; 
  public children: TreeNode[] = [];

  constructor(parent: TreeNode|null) 
  { 
    this.parent = parent; 
    if(this.parent) this.parent.children.push(this); 
  } 
}
So, when a new tree node is declared, if it isn’t a root, it will push itself to its parent’s children array. And with that, the children are all saved.



The item (circle 1) is the root. Its first child (circle 2) has a parent property whose value is… the parent (circle 1), whose children contain the first child (circle 2), and so on and on and on ad infinitum.

⚠️ Be aware! Now our data structure cannot be directly transformed into JSON. That is because although its shape continues to be closed, each child holds a reference to a parent, and that parent hold’s reference to its children. And that’s the recipe for “TypeError: Converting circular structure to JSON” because JSON does not know variables or pointers¹.

Applying the Tree
So far, we have a tree, but it contains no data. So, let’s apply it to a use case: a nested menu file explorer style, so each node will also have a label. I’ve written a code SandBox with an example that you can explore below:

To hold the label and other custom fields and methods, it is helpful to create a MenuItem class. But as we need the TreeNode functionality, MenuItem will extend TreeNode. Just like this:

export class MenuItem extends TreeNode 
{ 
  public label: string = ""; 
  public parent: MenuItem | null; 
  public children: MenuItem[] = []; 
 
  constructor(parent: MenuItem["parent"], label: string) 
  { 
    super(parent); 
    this.label = label; 
  } 
}
And to achieve this, it is also necessary to tweak TreeNode a little, so it handles extension better:

export class TreeNode implements TreeNodeInterface 
{ 
  public parent: TreeNodeInterface | null; 
  public children: TreeNodeInterface[] = [];

  constructor(parent: TreeNodeInterface | null)
  { 
    this.parent = parent; 
    if (this.parent) this.parent.children.push(this); 
  } 
}

export interface TreeNodeInterface 
{ 
  parent: TreeNodeInterface | null; 
  children: TreeNodeInterface[]; 
}
If we didn’t use the interface, MenuItem’s children and parents would be forced to TreeNode. But telling the compiler that these properties follow an interface does not lock them to TreeNode class.

Wrapping up
Traversable N-ary Trees can hold parent/children relationship information in an extensible way. Having a base class allows you to extend tree nodes to receive even more data as needed. It is suitable for hierarchical data, such as folders and files and nested menus.

And as each node knows its parent and its children, with recursion you can walk the tree up and down starting from any node. And that you’ll do on part 2 (to be released).