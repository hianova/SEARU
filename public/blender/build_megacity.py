
import bpy
import json
import os
import math

def build_city():
    # Clear existing objects
    bpy.ops.object.select_all(action='SELECT')
    bpy.ops.object.delete(use_global=False)
    
    # Load JSON data
    json_path = os.path.join(os.path.dirname(bpy.data.filepath) if bpy.data.filepath else os.getcwd(), "megacity_data.json")
    if not os.path.exists(json_path):
        json_path = "public/blender/megacity_data.json" # Fallback
        
    with open(json_path, 'r') as f:
        data = json.load(f)
        
    # Create Material
    mat = bpy.data.materials.new(name="MegaCity_Mat")
    mat.use_nodes = True
    bsdf = mat.node_tree.nodes["Principled BSDF"]
    
    pbr = data['material']
    bsdf.inputs['Base Color'].default_value = (pbr['albedo'][0], pbr['albedo'][1], pbr['albedo'][2], 1.0)
    bsdf.inputs['Roughness'].default_value = pbr['roughness']
    bsdf.inputs['Metallic'].default_value = pbr['metallic']
    
    # Build Rooms
    for i, r in enumerate(data['rooms']):
        # SVG coords (0,0 top-left) to Blender coords (0,0 center)
        x = (r['x'] - 250.0) / 10.0
        y = -(r['y'] - 250.0) / 10.0
        w = r['w'] / 10.0
        h = r['h'] / 10.0
        height = 5.0 + (i % 5) * 2.0 # Fake 3D height variation
        
        bpy.ops.mesh.primitive_cube_add(size=1, location=(x + w/2, y - h/2, height/2))
        obj = bpy.context.active_object
        obj.scale = (w, h, height)
        obj.name = f"Room_{r['name']}"
        if obj.data.materials:
            obj.data.materials[0] = mat
        else:
            obj.data.materials.append(mat)
            
    # Build Truss
    truss = data['truss']
    nodes = truss['nodes']
    for bar in truss['bars']:
        n1 = nodes[bar['node_a']]
        n2 = nodes[bar['node_b']]
        
        x1 = (n1['x'] - 50.0) / 2.0
        y1 = -(n1['y'] - 50.0) / 2.0
        x2 = (n2['x'] - 50.0) / 2.0
        y2 = -(n2['y'] - 50.0) / 2.0
        
        dx = x2 - x1
        dy = y2 - y1
        dist = math.sqrt(dx*dx + dy*dy)
        
        bpy.ops.mesh.primitive_cylinder_add(radius=bar['area'] / 20.0, depth=dist, location=(x1 + dx/2, y1 + dy/2, 10.0))
        obj = bpy.context.active_object
        
        # Rotate cylinder to align with nodes
        rot_y = math.pi / 2
        rot_z = math.atan2(dy, dx)
        obj.rotation_euler = (0, rot_y, rot_z)
        
    print("MegaCity generated successfully!")
    
if __name__ == "__main__":
    build_city()
