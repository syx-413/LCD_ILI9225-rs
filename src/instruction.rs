/// ILI9225 instructions.
#[derive(Debug, Clone, Copy)]
pub enum Instruction {
    DRIVER_OUTPUT_CTRL = 0x01,  // Driver Output Control
    LCD_AC_DRIVING_CTRL = 0x02, // LCD AC Driving Control
    ENTRY_MODE = 0x03,          // Entry Mode
    DISP_CTRL1 = 0x07,          // Display Control 1
    DISP_CTRL2 = 0x08,          // Blank Period Control
    FRAME_CYCLE_CTRL = 0x0B,    // Frame Cycle Control
    INTERFACE_CTRL = 0x0C,      // Interface Control
    OSC_CTRL = 0x0F,            // Osc Control
    //*************Power On sequence ****************//
    POWER_CTRL1 = 0x10,   // Power Control 1
    POWER_CTRL2 = 0x11,   // Power Control 2
    POWER_CTRL3 = 0x12,   // Power Control 3
    POWER_CTRL4 = 0x13,   // Power Control 4
    POWER_CTRL5 = 0x14,   // Power Control 5
    VCI_RECYCLING = 0x15, // VCI Recycling
    RAM_ADDR_SET1 = 0x20, // Horizontal GRAM Address Set
    RAM_ADDR_SET2 = 0x21, // Vertical GRAM Address Set
    GRAM_DATA_REG = 0x22, // GRAM Data Register
    //-------------- Set GRAM area -----------------//
    GATE_SCAN_CTRL = 0x30,          // Gate Scan Control Register
    VERTICAL_SCROLL_CTRL1 = 0x31,   // Vertical Scroll Control 1 Register
    VERTICAL_SCROLL_CTRL2 = 0x32,   // Vertical Scroll Control 2 Register
    VERTICAL_SCROLL_CTRL3 = 0x33,   // Vertical Scroll Control 3 Register
    PARTIAL_DRIVING_POS1 = 0x34,    // Partial Driving Position 1 Register
    PARTIAL_DRIVING_POS2 = 0x35,    // Partial Driving Position 2 Register
    HORIZONTAL_WINDOW_ADDR1 = 0x36, // Horizontal Address Start Position (Window-HStart)
    HORIZONTAL_WINDOW_ADDR2 = 0x37, // Horizontal Address End Position   (Window-HEnd)
    VERTICAL_WINDOW_ADDR1 = 0x38,   // Vertical Address Start Position	  (Window-VStart)
    VERTICAL_WINDOW_ADDR2 = 0x39,   // Vertical Address End Position	  (Window-VEnd)
    // ----------- Adjust the Gamma Curve ----------//
    GAMMA_CTRL1 = 0x50,  // Gamma Control 1
    GAMMA_CTRL2 = 0x51,  // Gamma Control 2
    GAMMA_CTRL3 = 0x52,  // Gamma Control 3
    GAMMA_CTRL4 = 0x53,  // Gamma Control 4
    GAMMA_CTRL5 = 0x54,  // Gamma Control 5
    GAMMA_CTRL6 = 0x55,  // Gamma Control 6
    GAMMA_CTRL7 = 0x56,  // Gamma Control 7
    GAMMA_CTRL8 = 0x57,  // Gamma Control 8
    GAMMA_CTRL9 = 0x58,  // Gamma Control 9
    GAMMA_CTRL10 = 0x59, // Gamma Control 10

    // delay 50ms
    // misc
    COLOR_BLACK = 0x000000,
    COLOR_WHITE = 0xFFFFFF,
}
// const UINT16  regValues[] =
// {
// 	=0x01, =0x011C,
// 	=0x02, =0x0100,
// 	=0x03, =0x1030,
// 	=0x08, =0x0808, // set BP and FP
// 	=0x0B, =0x1100, // frame cycle
// 	=0x0C, =0x0000, // RGB interface setting R0Ch==0x0110 for RGB 18Bit and R0Ch=0111for RGB16Bit
// 	=0x0F, =0x1401, // Set frame rate----0801
// 	=0x15, =0x0000, // set system interface
// 	=0x20, =0x0000, // Set GRAM Address
// 	=0x21, =0x0000, // Set GRAM Address
// 	//*************Power On sequence ****************//
// 	TFTLCD_DELAY16, 50, // delay 50ms
// 	=0x10, =0x0800, // Set SAP,DSTB,STB----0A00
// 	=0x11, =0x1F3F, // Set APON,PON,AON,VCI1EN,VC----1038
// 	TFTLCD_DELAY16, 50, // delay 50ms
// 	=0x12, =0x0121, // Internal reference voltage= Vci;----1121
// 	=0x13, =0x006F, // Set GVDD----0066
// 	=0x14, =0x4349, // Set VCOMH/VCOML voltage----5F60
// 	//-------------- Set GRAM area -----------------//
// 	=0x30, =0x0000,
// 	=0x31, =0x00DB,
// 	=0x32, =0x0000,
// 	=0x33, =0x0000,
// 	=0x34, =0x00DB,
// 	=0x35, =0x0000,
// 	=0x36, =0x00AF,
// 	=0x37, =0x0000,
// 	=0x38, =0x00DB,
// 	=0x39, =0x0000,
// 	// ----------- Adjust the Gamma Curve ----------//
// 	=0x50, =0x0001, // =0x0400
// 	=0x51, =0x200B, // =0x060B
// 	=0x52, =0x0000, // =0x0C0A
// 	=0x53, =0x0404, // =0x0105
// 	=0x54, =0x0C0C, // =0x0A0C
// 	=0x55, =0x000C, // =0x0B06
// 	=0x56, =0x0101, // =0x0004
// 	=0x57, =0x0400, // =0x0501
// 	=0x58, =0x1108, // =0x0E00
// 	=0x59, =0x050C, // =0x000E
// 	TFTLCD_DELAY16, 50, // delay 50ms
// 	=0x07, =0x1017,
// 	=0x22, =0x0000,
// };
