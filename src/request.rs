use sdf;

static mut g_writebuf: [c_char; sdf::BLK_SZ] = [0; ARRAY_SIZE];
static mut g_metabuf: [c_char; sdf::BLK_SZ_META] = [0; ARRAY_SIZE];

