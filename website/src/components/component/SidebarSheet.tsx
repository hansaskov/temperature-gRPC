import { Sheet, SheetTrigger, SheetContent } from "@/components/ui/sheet";
import { Button } from "../ui/button";
import { MenuIcon } from "./Icons";

const SidebarSheet = ({ children }: {children: React.ReactNode}) => {
  return (
    <Sheet>
      <SheetTrigger asChild>
        <Button variant="outline" size="icon" className="shrink-0 md:hidden">
          <MenuIcon className="h-5 w-5" />
          <span className="sr-only">Toggle navigation menu</span>
        </Button>
      </SheetTrigger>
      <SheetContent side="left"  className="">
        {children}
      </SheetContent>
    </Sheet>
  );
};

export default SidebarSheet;